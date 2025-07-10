//! Example usage of the OpenAI response API.

// std
use std::{env, error::Error};
// crates.io
use futures::StreamExt;
use tracing_subscriber::EnvFilter;
// self
use openagent::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

	let _ = dotenvy::dotenv();
	let api = Api::new(Auth {
		uri: "https://api.openai.com/v1".into(),
		key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set; qed"),
	});
	let req = ResponseRequest {
		input: Either::A("Hello, how are you?".into()),
		model: Model::Gpt4oMini,
		// model: Model::Custom {
		// 	id: "".into(),
		// 	name: "".into(),
		// 	embedding: false,
		// 	reasoning: false,
		// 	function_calling: false,
		// },
		..Default::default()
	};

	println!("request: {}", serde_json::to_string(&req)?);

	// Example 1: Non-streaming response.
	println!("=== non-streaming response ===");
	println!("{:#?}", api.create_response(req.clone()).await);

	// Example 2: Streaming response with typed event handler.
	println!("\n=== streaming response with typed events ===");

	match api
		.create_response_stream(
			req.clone(),
			SseOptions::new(ApiEventHandler::default()).drop_event(true),
		)
		.await
	{
		Ok(mut stream) =>
			while let Some(event_res) = stream.next().await {
				match event_res {
					Ok(event) => match event {
						ResponseEvent::Created(e) => {
							println!("🚀 response created: {e:?}");
						},
						ResponseEvent::InProgress(e) => {
							println!("⏳ response in progress: {e:?}");
						},
						ResponseEvent::OutputItemAdded(e) => {
							println!("📝 output item added: {e:?}");
						},
						ResponseEvent::ContentPartAdded(e) => {
							println!("📄 content part added: {e:?}");
						},
						ResponseEvent::OutputTextDelta(e) => {
							print!("{}", e.delta);
						},
						ResponseEvent::OutputTextDone(e) => {
							println!("\n✅ text output done: {e:?}");
						},
						ResponseEvent::ContentPartDone(e) => {
							println!("✅ content part done: {e:?}");
						},
						ResponseEvent::OutputItemDone(e) => {
							println!("✅ output item done: {e:?}");
						},
						ResponseEvent::Completed(e) => {
							println!("🎉 response completed: {e:?}");
							println!(
								"📊 usage: {} input + {} output = {} total tokens",
								e.response.usage.as_ref().unwrap().input_tokens,
								e.response.usage.as_ref().unwrap().output_tokens,
								e.response.usage.as_ref().unwrap().total_tokens
							);
						},
						_ => (),
					},
					Err(e) => {
						println!("❌ stream error: {e:#?}");

						break;
					},
				}
			},
		Err(e) => {
			println!("❌ error: {e:#?}");
		},
	}

	// Example 3: Custom event handler for accumulating text.
	println!("\n=== custom event handler example ===");

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
				// model: Model::Custom {
				// 	id: "".into(),
				// 	name: "".into(),
				// 	embedding: false,
				// 	reasoning: false,
				// 	function_calling: false,
				// },
				input: Either::B(vec![ResponseInput::Message(ResponseMessage {
					content: Either::B(vec![
						ResponseMessageInputContent::InputText {
							text: "Come up with keywords related to the image, and search on the web using the search tool for any news related to the keywords, summarize the findings and cite the sources.".into(),
						},
						ResponseMessageInputContent::InputImage {
							detail: Default::default(),
							file_id: Default::default(),
							image_url: Some("https://upload.wikimedia.org/wikipedia/commons/thumb/1/15/Cat_August_2010-4.jpg/2880px-Cat_August_2010-4.jpg".into()),
						},
					]),
					role: Role::User,
				})]),
				tools: Some(vec![Tool::WebSearchPreview {
					search_context_size: Default::default(),
					user_location: Default::default(),
				}]),
				..Default::default()
			},
			SseOptions::new(ApiEventHandler::default()).drop_event(true),
		)
		.await
	{
		Ok(mut stream) =>
			while let Some(event_result) = stream.next().await {
				match event_result {
					Ok(event) => {
						accumulator.handle_event(&event);
					},
					Err(e) => {
						println!("❌ stream error: {e:#?}");

						break;
					},
				}
			},
		Err(e) => {
			println!("❌ error: {e:#?}");
		},
	}

	Ok(())
}
