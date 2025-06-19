// crates.io
use futures::{future::BoxFuture, stream::BoxStream};
// self
use crate::_prelude::*;

/// Core trait for implementing tools that the agent can use.
///
/// Tools are the building blocks that allow the agent to interact with external systems,
/// perform calculations, retrieve information, or execute actions.
pub trait ToolT
where
	Self: Send + Sync,
{
	/// Unique identifier for the tool.
	fn name(&self) -> &str;

	/// Human-readable description of what the tool does.
	fn description(&self) -> &str;

	/// JSON schema describing the expected parameters.
	fn schema(&self) -> Value;

	/// Execute the tool with given parameters.
	fn call(&self, params: Value) -> BoxFuture<'static, Result<Value>>;

	/// Streaming execution for tools that provide incremental results.
	fn call_stream(
		&self,
		#[allow(unused)] params: Value,
	) -> BoxFuture<'static, Result<BoxStream<'static, String>>> {
		let tool = self.name().to_owned();

		// Does not support streaming by default.
		Box::pin(async move { Err(ToolError::StreamingNotSupported(tool))? })
	}

	/// Check if the tool supports native streaming execution.
	///
	/// Tools should override this to return true if they provide native streaming support.
	fn supports_stream(&self) -> bool {
		false
	}
}

/// Represents a request to call a tool with specific parameters.
#[derive(Clone, Debug)]
pub struct ToolCall {
	/// The name of the tool to call.
	pub name: String,
	/// The arguments to pass to the tool.
	pub args: Value,
}

/// Represents the result of a tool call.
#[derive(Clone, Debug)]
pub struct ToolCallResult {
	/// The tool call that was executed.
	pub tool_call: ToolCall,
	/// The outcome of the tool call, which can be either success or error.
	pub outcome: ToolCallOutcome,
}
// impl ToolCallResult {
// 	pub(crate) fn success(tool: String, args: Value, result: Value) -> Self {
// 		Self {
// 			tool_call: ToolCall { name: tool, args },
// 			outcome: ToolCallOutcome::Success { result },
// 		}
// 	}

// 	pub(crate) fn err(tool: String, args: Value, message: String) -> Self {
// 		Self {
// 			tool_call: ToolCall { name: tool, args },
// 			outcome: ToolCallOutcome::Error { message },
// 		}
// 	}
// }

#[derive(Clone, Debug)]
pub enum ToolCallOutcome {
	Success { result: Value },
	Error { message: String },
}
