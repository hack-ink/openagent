//! Example usage of the OpenAI chat API.

// std
use std::{
	env,
	error::Error,
	io::{self, Write},
};
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
		uri: "https://openrouter.ai/api/v1".into(),
		key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set; qed"),
	});
	let req = ChatRequest {
		messages: vec![
			ChatMessage::System(ChatMessageCommon {
				content: Either::A("You're a helpful assistant.".into()),
				name: None,
			}),
			ChatMessage::User(ChatMessageCommon {
				content: Either::A("What is the capital of France?".into()),
				name: None,
			}),
		],
		..Default::default()
	};

	// Example 1: Non-streaming chat.
	println!("=== non-streaming chat ===");
	let res = api.create_chat(req.clone()).await;
	println!("{res:#?}");

	// Example 2: Streaming chat.
	println!("\n=== streaming chat ===");
	match ApiChat::create_chat_stream(
		&api,
		req,
		SseOptions::new(<ApiEventHandler<ChatChunkObject>>::default()).drop_event(true),
	)
	.await
	{
		Ok(mut stream) => {
			print!("🤖 ");

			while let Some(event_result) = stream.next().await {
				match event_result {
					Ok(chunk) =>
						for choice in chunk.choices {
							if let Some(delta) = &choice.delta {
								if let Some(content) = &delta.content {
									print!("{content}");

									io::stdout().flush()?;
								}
							}
						},
					Err(e) => {
						println!("\n❌ stream error: {e}");

						break;
					},
				}
			}

			// New line after streaming.
			println!();
		},
		Err(e) => {
			println!("❌ error creating stream: {e}");
		},
	}

	Ok(())
}
