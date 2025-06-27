//! OpenAI API common types.

// std
use std::error::Error as ErrorT;
// self
use crate::_prelude::*;

impl_const_str! {
	Function  => "function",
}

impl_serializable_deserializable_enum! {
	Role {
		User => "user",
		Assistant => "assistant",
		System => "system",
		Developer => "developer"
	}
}

impl_serializable_deserializable_enum! {
	ReasoningEffort {
		Low => "low",
		Medium => "medium",
		High => "high"
	}
}

impl_serializable_deserializable_enum! {
	ServiceTier {
		Auto => "auto",
		Default => "default",
		Flex => "flex",
	}
}

impl_serializable_enum! {
	ImageDetail {
		High => "high",
		Low => "low",
		#[default]
		Auto => "auto",
	}
}

impl_serializable_enum! {
	Purpose {
		Assistants => "assistants",
		Batch => "batch",
		FineTune => "fine-tune",
		Vision => "vision",
		UserData => "user_data",
		Evals => "evals"
	}
}

/// Represents either a successful API response or an error response.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
	/// Successful API response containing the expected data.
	Ok(T),
	/// Error response containing error details.
	Err(ApiErrorWrapper),
}
impl<T> ApiResult<T> {
	/// Converts the API result into a standard Result type.
	pub fn as_result(self) -> Result<T, ApiError> {
		match self {
			Self::Ok(t) => Ok(t),
			Self::Err(e) => Err(e.error),
		}
	}
}

/// Represents a value that can be one of two different types.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<A, B> {
	/// First type variant.
	A(A),
	/// Second type variant.
	B(B),
}
impl<A, B> Default for Either<A, B>
where
	A: Default,
{
	/// Creates a default instance using the first type's default value.
	fn default() -> Self {
		Self::A(A::default())
	}
}

/// Wrapper structure for API error responses.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiErrorWrapper {
	/// The actual error information from the API.
	pub error: ApiError,
}

/// Represents an error returned by the OpenAI API.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiError {
	/// The specific type of error encountered.
	pub r#type: String,
	/// Common error fields shared across all error types.
	#[serde(flatten)]
	pub base: ErrorBase,
}
impl Display for ApiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {} (code: {}, param: {})",
			self.r#type,
			self.base.message,
			self.base.code.as_deref().unwrap_or("N/A"),
			self.base.param.as_deref().unwrap_or("N/A")
		)
	}
}
impl ErrorT for ApiError {}

/// Contains the basic error information common to all API errors.
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorBase {
	/// Human-readable description of the error.
	pub message: String,
	/// Optional error code identifying the specific error type.
	pub code: Option<String>,
	/// Optional parameter name that caused the error.
	pub param: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logprobs {
	#[serde(flatten)]
	pub logprob: Logprob,
	pub top_logprobs: Vec<Logprob>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logprob {
	pub bytes: Vec<u8>,
	pub logprob: f32,
	pub token: String,
}
