//! OpenAI responses API.
//!
//! <https://platform.openai.com/docs/api-reference/responses>

// self
use crate::_prelude::*;

mod create;
pub use create::*;

mod event;
pub use event::*;

mod object;
pub use object::*;

mod r#type;
pub use r#type::*;

/// OpenAI responses API.
pub trait ApiResponse
where
	Self: ApiBase,
{
	/// Create a response (non-streaming).
	fn create_response(
		&self,
		mut request: ResponseRequest,
	) -> impl Send + Future<Output = Result<ResponseObject>> {
		async {
			// Ensure stream is disabled for non-streaming.
			request.stream = None;

			let resp = self.post_json("/responses", request).await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<ResponseObject>>(&resp)?.as_result()?)
		}
	}

	/// Create a response with streaming.
	fn create_response_stream<H>(
		&self,
		mut request: ResponseRequest,
		options: SseOptions<H>,
	) -> impl Send + Future<Output = Result<EventStream<H::Event>>>
	where
		H: 'static + EventHandler,
	{
		async move {
			// Ensure stream is enabled for streaming.
			request.stream = Some(true);

			self.sse("/responses", request, options).await
		}
	}
}
impl<T> ApiResponse for T where T: ApiBase {}
