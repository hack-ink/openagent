//! Example usage of the OpenAI batches API.

// std
use std::{env, error::Error};
// crates.io
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
	let res = api
		.upload_file(
			"foo.jsonl",
			vec![
				BatchInput {
					custom_id: "0".into(),
					method: Default::default(),
					url: Endpoint::Embeddings,
					body: EmbeddingRequest {
						input: Either::A("Foo".into()),
						model: Model::TextEmbedding3Large,
						..Default::default()
					},
				},
				BatchInput {
					custom_id: "1".into(),
					method: Default::default(),
					url: Endpoint::Embeddings,
					body: EmbeddingRequest {
						input: Either::A("Bar".into()),
						model: Model::TextEmbedding3Large,
						..Default::default()
					},
				},
			]
			.into_iter()
			.map(|input| serde_json::to_string(&input).expect("serialization must succeed; qed"))
			.collect::<Vec<_>>()
			.join("\n")
			.as_bytes()
			.to_vec(),
			Purpose::Batch,
		)
		.await;

	println!("{res:#?}");

	let req = api.retrieve_file_content("file-8TSU8J5RNWNHWnmjKyFGFe2b").await;
	let req = BatchRequest {
		endpoint: Endpoint::Embeddings,
		input_file_id: res?.id,
		..Default::default()
	};
	let res = api.create_batch(req).await;

	println!("{res:#?}");

	let res = api.retrieve_batch(&res?.id).await;

	println!("{res:#?}");

	Ok(())
}
