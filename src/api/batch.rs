//! OpenAI Batches API
//!
//! <https://platform.openai.com/docs/api-reference/batch>

// self
use crate::_prelude::*;

/// OpenAI batches API.
pub trait ApiBatch
where
	Self: ApiBase,
{
	/// Create a batch.
	fn create_batch(
		&self,
		request: BatchRequest,
	) -> impl Send + Future<Output = Result<BatchObject>> {
		async {
			let resp = self.post_json("/batches", request).await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<BatchObject>>(&resp)?.as_result()?)
		}
	}

	/// Retrieve a batch by ID.
	fn retrieve_batch(&self, id: &str) -> impl Send + Future<Output = Result<BatchObject>> {
		async move {
			let resp = self.get(&format!("/batches/{id}")).await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<BatchObject>>(&resp)?.as_result()?)
		}
	}
}
impl<T> ApiBatch for T where T: ApiBase {}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Serialize)]
pub struct BatchRequest {
	pub completion_window: Const24H,
	pub endpoint: Endpoint,
	pub input_file_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Map>,
}

impl_const_str! {
	24H => "24h",
}

impl_serializable_deserializable_enum! {
	Endpoint {
		#[default]
		Response => "/v1/response",
		ChatCompletion => "/v1/chat/completions",
		Embeddings => "/v1/embeddings",
		Completions => "/v1/completions",
	}
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct BatchInput<T> {
	pub custom_id: String,
	pub method: ConstPost,
	pub url: Endpoint,
	pub body: T,
}

impl_const_str! {
	Post => "POST",
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct BatchObject {
	pub cancelled_at: Option<u64>,
	pub cancelling_at: Option<u64>,
	pub completed_at: Option<u64>,
	// Can be ignored.
	// pub completion_window: Const24H,
	pub created_at: u64,
	pub endpoint: Endpoint,
	pub error_file_id: Option<String>,
	pub errors: Option<BatchError>,
	pub expired_at: Option<u64>,
	pub expires_at: u64,
	pub failed_at: Option<u64>,
	pub finalizing_at: Option<u64>,
	pub id: String,
	pub in_progress_at: Option<u64>,
	pub input_file_id: String,
	pub metadata: Option<Map>,
	// Can be ignored.
	// pub object: ConstBatch,
	pub output_file_id: Option<String>,
	pub request_counts: RequestCounts,
	pub status: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct BatchError {
	pub data: Vec<ErrorData>,
	// Can be ignored.
	// pub object: ConstList,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorData {
	#[serde(flatten)]
	pub base: ErrorBase,
	pub line: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct RequestCounts {
	pub completed: u32,
	pub failed: u32,
	pub total: u32,
}
