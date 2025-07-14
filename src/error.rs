#![allow(missing_docs)]

// std
use std::time::Duration;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("{0}")]
	Any(String),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Codec(#[from] tokio_util::codec::LinesCodecError),
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),

	#[error(transparent)]
	Agent(#[from] AgentError),
	#[error(transparent)]
	Api(#[from] crate::api::r#type::ApiError),
	#[error("timeout after {0:?}")]
	Timeout(Duration),
	#[error(transparent)]
	Tool(#[from] ToolError),
}
impl Error {
	pub fn any<T>(any: T) -> Self
	where
		T: Into<String>,
	{
		Self::Any(any.into())
	}
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
	#[error("maximum steps {0} reached without final answer")]
	MaxStepsExceeded(usize),
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
	#[error("tool '{0}' does not support streaming")]
	StreamingNotSupported(String),
	#[error("unknown tool: {0}")]
	Unknown(String),
}
