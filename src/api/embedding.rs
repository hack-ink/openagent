//! OpenAI Embeddings API
//!
//! <https://platform.openai.com/docs/api-reference/embeddings>

// self
use crate::_prelude::*;

/// OpenAI embeddings API.
pub trait ApiEmbedding
where
	Self: ApiBase,
{
	/// Create an embedding.
	fn create_embedding(
		&self,
		request: EmbeddingRequest,
	) -> impl Send + Future<Output = Result<EmbeddingResponse>> {
		async {
			let resp = self.post_json("/embeddings", request).await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<EmbeddingResponse>>(&resp)?.as_result()?)
		}
	}
}
impl<T> ApiEmbedding for T where T: ApiBase {}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct EmbeddingRequest {
	pub input: Either<String, Vec<String>>,
	pub model: Model,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<u32>,
	pub encoding_format: Option<EncodingFormat>,
	pub user: Option<String>,
}
impl Default for EmbeddingRequest {
	fn default() -> Self {
		Self {
			input: Either::A("".into()),
			model: Model::TextEmbedding3Large,
			dimensions: None,
			encoding_format: None,
			user: None,
		}
	}
}

impl_serializable_enum! {
	EncodingFormat {
		#[default]
		Float => "float",
		Base64 => "base64",
	}
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct EmbeddingResponse {
	// Can be ignored.
	// pub object: ConstList,
	pub data: Vec<EmbeddingObject>,
	pub model: Model,
	pub usage: EmbeddingUsage,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct EmbeddingObject {
	pub embedding: Vec<f32>,
	pub index: u32,
	// Can be ignored.
	// pub object: ConstEmbedding,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct EmbeddingUsage {
	pub prompt_tokens: u32,
	pub total_tokens: u32,
}
