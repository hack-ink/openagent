//! Example usage of the OpenAI MCP API.

// std
use std::{env, error::Error};
// crates.io
use futures::StreamExt;
use rmcp::{
	ServiceExt,
	model::{ClientInfo, Implementation},
	transport::SseClientTransport,
};
use tracing_subscriber::EnvFilter;
// self
use openagent::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
	dotenvy::dotenv().expect(".env must be loaded; qed");

	let api = Api::new(Auth {
		uri: "https://api.openai.com/v1".into(),
		key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set; qed"),
	});
	let transport = SseClientTransport::start("http://0.0.0.0:8000/sse").await?;
	let mcp_info = ClientInfo {
		protocol_version: Default::default(),
		capabilities: Default::default(),
		client_info: Implementation { name: "postgresql".into(), version: "0.1.0".into() },
	};
	let mcp = mcp_info.serve(transport).await?;
	let tools = mcp.list_tools(Default::default()).await?;
	let tools = tools
		.tools
		.into_iter()
		.map(|t| {
			let parameters = t.schema_as_json_value();

			Tool::Function {
				name: t.name.into_owned(),
				parameters,
				strict: Default::default(),
				description: t.description.map(|d| d.into_owned()),
			}
		})
		.collect();

	// println!("available tools: {tools:?}");

	struct TextAccumulator {
		content: String,
	}
	impl TextAccumulator {
		fn new() -> Self {
			Self { content: String::new() }
		}

		fn handle_event(&mut self, event: &ResponseEvent) {
			match event {
				ResponseEvent::Created(e) => {
					println!("📋 starting response: {}", e.response.id);
				},
				ResponseEvent::OutputTextDelta(e) => {
					self.content.push_str(&e.delta);

					print!("{}", e.delta);
				},
				ResponseEvent::Completed(e) => {
					println!("\n📝 final accumulated text ({} chars):", self.content.len());
					println!("\"{}\"", self.content);
					println!(
						"📊 total tokens used: {}",
						e.response.usage.as_ref().unwrap().total_tokens
					);
				},
				_ => (),
			}
		}
	}

	let mut accumulator = TextAccumulator::new();

	match api
		.create_response_stream(
			ResponseRequest {
				model: Model::Gpt4oMini,
				input: Either::B(vec![ResponseInput::Message(ResponseMessage {
					content: Either::B(vec![ResponseMessageInputContent::InputText {
					text:
						"Select all APY data from the PostgreSQL DB's \"defi_apy_history\" table."
							.into(),
				}]),
					role: Role::User,
				})]),
				tools: Some(tools),
				tool_choice: Some(ToolChoice::Mode(ToolChoiceMode::Auto)),
				..Default::default()
			},
			SseOptions::new(ApiEventHandler::default()).drop_event(true),
		)
		.await
	{
		Ok(mut stream) =>
			while let Some(event_result) = stream.next().await {
				println!("{event_result:?}");

				match event_result {
					Ok(event) => {
						accumulator.handle_event(&event);
					},
					Err(e) => {
						println!("❌ stream error: {e}");

						break;
					},
				}
			},
		Err(e) => {
			println!("❌ error: {e}");
		},
	}

	mcp.cancel().await?;

	Ok(())
}
