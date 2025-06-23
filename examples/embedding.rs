//! Example usage of the OpenAI embeddings API.

// std
use std::{env, error::Error};
// crates.io
use tracing_subscriber::EnvFilter;
// self
use openagent::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
	dotenvy::dotenv().expect(".env must be loaded; qed");

	let api = Api::new(Auth {
		uri: env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL must be set; qed"),
		key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set; qed"),
	});
	let req = EmbeddingRequest {
		input: Either::A("Hello, how are you?".into()),
		model: Model::Unknown("Qwen/Qwen3-Embedding-4B".into()),
		encoding_format: Some(EncodingFormat::Float),
		..Default::default()
	};
	let res = api.create_embedding(req).await;

	println!("{res:#?}");

	Ok(())
}
