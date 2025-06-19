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

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
	Ok(T),
	Err(ApiErrorWrapper),
}
impl<T> ApiResult<T> {
	pub fn as_result(self) -> Result<T, ApiError> {
		match self {
			Self::Ok(t) => Ok(t),
			Self::Err(e) => Err(e.error),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<A, B> {
	A(A),
	B(B),
}
impl<A, B> Default for Either<A, B>
where
	A: Default,
{
	fn default() -> Self {
		Self::A(A::default())
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApiErrorWrapper {
	pub error: ApiError,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApiError {
	// Put the type as the first field for better error messages in debugging.
	pub r#type: String,
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

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorBase {
	pub message: String,
	pub code: Option<String>,
	pub param: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logprobs {
	#[serde(flatten)]
	pub logprob: Logprob,
	pub top_logprobs: Vec<Logprob>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logprob {
	pub bytes: Vec<u8>,
	pub logprob: f32,
	pub token: String,
}
