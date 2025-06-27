//! OpenAI Chat API
//!
//! <https://platform.openai.com/docs/api-reference/chat>

// self
use crate::_prelude::*;

/// OpenAI chat1 API.
pub trait ApiChat
where
	Self: ApiBase,
{
	/// Create a chat.
	fn create_chat(
		&self,
		mut request: ChatRequest,
	) -> impl Send + Future<Output = Result<ChatObject>> {
		async {
			// Ensure stream is disabled for non-streaming.
			request.stream = None;
			request.stream_options = None;

			let resp = self.post_json("/chat/completions", request).await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<ChatObject>>(&resp)?.as_result()?)
		}
	}

	/// Create a chat with streaming.
	fn create_chat_stream<H>(
		&self,
		mut request: ChatRequest,
		options: SseOptions<H>,
	) -> impl Send + Future<Output = Result<EventStream<H::Event>>>
	where
		H: 'static + EventHandler,
	{
		async move {
			// Ensure stream is enabled for streaming.
			request.stream = Some(true);
			request.stream_options = Some(StreamOptions { include_usage: Some(true) });

			self.sse("/chat/completions", request, options).await
		}
	}
}
impl<T> ApiChat for T where T: ApiBase {}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChatRequest {
	pub messages: Vec<ChatMessage>,
	pub model: Model,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub frequency_penalty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logit_bias: Option<Map>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Map>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prediction: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub presence_penalty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<ChatResponseFormat>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<ServiceTier>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_options: Option<StreamOptions>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web_search_options: Option<Value>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatMessage {
	Developer(ChatMessageCommon<Either<String, Vec<ChatMessageContentText>>>),
	System(ChatMessageCommon<Either<String, Vec<ChatMessageContentText>>>),
	User(ChatMessageCommon<Either<String, Vec<ChatMessageContentMultimedia>>>),
	Assistant(ChatMessageAssistant),
	Tool(ChatMessageTool),
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ChatMessageCommon<T> {
	pub content: T,
	pub name: Option<String>,
}
impl<A, B> Default for ChatMessageCommon<Either<A, B>>
where
	A: Default,
{
	fn default() -> Self {
		ChatMessageCommon { content: Either::A(Default::default()), name: Default::default() }
	}
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ChatMessageContentText {
	pub text: String,
	pub r#type: ConstText,
}

impl_const_str! {
	Text => "text"
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatMessageContentMultimedia {
	Text(String),
	InputImage { image_url: ImageUrl },
	InputAudio { input_audio: InputAudio },
	File { file_data: Option<String>, file_id: Option<String>, filename: Option<String> },
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ImageUrl {
	pub url: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<ImageDetail>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct InputAudio {
	pub data: String,
	pub format: AudioFormat,
}

impl_serializable_enum! {
	AudioFormat {
		Wav => "wav",
		Mp3 => "mp3",
	}
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChatMessageAssistant {
	#[serde(flatten)]
	pub common: ChatMessageCommon<
		Either<String, Vec<Either<ChatMessageContentText, ChatMessageContentRefusal>>>,
	>,
	pub audio: Option<Audio>,
	pub refusal: Option<String>,
	pub tool_calls: Option<Vec<ChatToolCall>>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ChatMessageContentRefusal {
	pub refusal: String,
	pub r#type: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct Audio {
	pub id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatToolCall {
	pub function: Function,
	pub id: String,
	pub r#type: ConstFunction,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
	pub arguments: Value,
	pub name: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ChatMessageTool {
	pub content: Either<String, Vec<ChatMessageContentText>>,
	pub tool_call_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatResponseFormat {
	Text,
	JsonSchema { json_schema: ChatResponseFormatJsonSchema },
	JsonObject,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct ChatResponseFormatJsonSchema {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub struct StreamOptions {
	pub include_usage: Option<bool>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatObject {
	pub choices: Vec<ChatChoice>,
	pub created: u64,
	pub id: String,
	pub model: Model,
	// Can be ignored.
	// pub object:
	pub service_tier: Option<ServiceTier>,
	pub system_fingerprint: String,
	pub usage: ChatUsage,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatChoice {
	pub finish_reason: String,
	pub index: u32,
	pub logprobs: Option<Logprobs>,
	pub message: ChatChoiceMessage,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatLogprobs {
	pub content: Option<Vec<Logprobs>>,
	pub refusal: Option<Vec<Logprobs>>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatChoiceMessage {
	pub content: Option<String>,
	pub refusal: Option<String>,
	pub role: Role,
	pub annotations: Option<Vec<Value>>,
	pub audio: Option<Value>,
	pub tool_calls: Option<Vec<Value>>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatUsage {
	pub completion_tokens: u32,
	pub prompt_tokens: u32,
	pub total_tokens: u32,
	pub completion_tokens_details: ChatCompletionTokensDetails,
	pub prompt_tokens_details: ChatPromptTokensDetails,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatCompletionTokensDetails {
	pub accepted_prediction_tokens: Option<u32>,
	pub audio_tokens: Option<u32>,
	pub reasoning_tokens: u32,
	pub rejected_prediction_tokens: Option<u32>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatPromptTokensDetails {
	pub audio_tokens: Option<u32>,
	pub cached_tokens: u32,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatChunkObject {
	pub choices: Vec<ChatChunkChoice>,
	pub created: u64,
	pub id: String,
	pub model: Model,
	// Can be ignored.
	// pub object: String,
	pub service_tier: Option<ServiceTier>,
	pub system_fingerprint: Option<String>,
	pub usage: Option<ChatUsage>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatChunkChoice {
	pub delta: Option<ChatChunkChoiceDelta>,
	pub finish_reason: Option<String>,
	pub index: u32,
	pub logprobs: Option<ChatLogprobs>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatChunkChoiceDelta {
	pub content: Option<String>,
	pub refusal: Option<String>,
	pub role: Option<String>,
	pub tool_calls: Option<Vec<ChatToolCall>>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct ChatToolCallIndexed {
	pub index: u32,
	#[serde(flatten)]
	pub tool_call: ChatToolCall,
}
