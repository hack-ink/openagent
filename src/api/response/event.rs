//! Event types for OpenAI Response SSE API.

#![allow(missing_docs)]

// self
use super::{object::*, r#type::*};
use crate::_prelude::*;

/// All possible events from the OpenAI Response API stream.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseEvent {
	#[serde(rename = "response.created")]
	Created(ResponseCreatedEvent),
	#[serde(rename = "response.in_progress")]
	InProgress(ResponseInProgressEvent),
	#[serde(rename = "response.completed")]
	Completed(ResponseCompletedEvent),
	#[serde(rename = "response.failed")]
	Failed(ResponseFailedEvent),
	#[serde(rename = "response.incomplete")]
	Incomplete(ResponseIncompleteEvent),
	#[serde(rename = "response.queued")]
	Queued(ResponseQueuedEvent),
	#[serde(rename = "response.output_item.added")]
	OutputItemAdded(ResponseOutputItemAddedEvent),
	#[serde(rename = "response.output_item.done")]
	OutputItemDone(ResponseOutputItemDoneEvent),
	#[serde(rename = "response.content_part.added")]
	ContentPartAdded(ResponseContentPartAddedEvent),
	#[serde(rename = "response.content_part.done")]
	ContentPartDone(ResponseContentPartDoneEvent),
	#[serde(rename = "response.output_text.delta")]
	OutputTextDelta(ResponseOutputTextDeltaEvent),
	#[serde(rename = "response.output_text.done")]
	OutputTextDone(ResponseOutputTextDoneEvent),
	#[serde(rename = "response.refusal.delta")]
	RefusalDelta(ResponseRefusalDeltaEvent),
	#[serde(rename = "response.refusal.done")]
	RefusalDone(ResponseRefusalDoneEvent),
	#[serde(rename = "response.function_call_arguments.delta")]
	FunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDeltaEvent),
	#[serde(rename = "response.function_call_arguments.done")]
	FunctionCallArgumentsDone(ResponseFunctionCallArgumentsDoneEvent),
	#[serde(rename = "response.file_search_call.in_progress")]
	FileSearchCallInProgress(ResponseFileSearchCallInProgressEvent),
	#[serde(rename = "response.file_search_call.searching")]
	FileSearchCallSearching(ResponseFileSearchCallSearchingEvent),
	#[serde(rename = "response.file_search_call.completed")]
	FileSearchCallCompleted(ResponseFileSearchCallCompletedEvent),
	#[serde(rename = "response.web_search_call.in_progress")]
	WebSearchCallInProgress(ResponseWebSearchCallInProgressEvent),
	#[serde(rename = "response.web_search_call.searching")]
	WebSearchCallSearching(ResponseWebSearchCallSearchingEvent),
	#[serde(rename = "response.web_search_call.completed")]
	WebSearchCallCompleted(ResponseWebSearchCallCompletedEvent),
	#[serde(rename = "response.reasoning_summary_part.added")]
	ReasoningSummaryPartAdded(ResponseReasoningSummaryPartAddedEvent),
	#[serde(rename = "response.reasoning_summary_part.done")]
	ReasoningSummaryPartDone(ResponseReasoningSummaryPartDoneEvent),
	#[serde(rename = "response.reasoning_summary_text.delta")]
	ReasoningSummaryTextDelta(ResponseReasoningSummaryTextDeltaEvent),
	#[serde(rename = "response.reasoning_summary_text.done")]
	ReasoningSummaryTextDone(ResponseReasoningSummaryTextDoneEvent),
	#[serde(rename = "response.image_generation_call.completed")]
	ImageGenerationCallCompleted(ResponseImageGenerationCallCompletedEvent),
	#[serde(rename = "response.image_generation_call.generating")]
	ImageGenerationCallGenerating(ResponseImageGenerationCallGeneratingEvent),
	#[serde(rename = "response.image_generation_call.in_progress")]
	ImageGenerationCallInProgress(ResponseImageGenerationCallInProgressEvent),
	#[serde(rename = "response.image_generation_call.partial_image")]
	ImageGenerationCallPartialImage(ResponseImageGenerationCallPartialImageEvent),
	#[serde(rename = "response.mcp_call.arguments.delta")]
	McpCallArgumentsDelta(ResponseMcpCallArgumentsDeltaEvent),
	#[serde(rename = "response.mcp_call.arguments.done")]
	McpCallArgumentsDone(ResponseMcpCallArgumentsDoneEvent),
	#[serde(rename = "response.mcp_call.completed")]
	McpCallCompleted(ResponseMcpCallCompletedEvent),
	#[serde(rename = "response.mcp_call.failed")]
	McpCallFailed(ResponseMcpCallFailedEvent),
	#[serde(rename = "response.mcp_call.in_progress")]
	McpCallInProgress(ResponseMcpCallInProgressEvent),
	#[serde(rename = "response.mcp_list_tools.completed")]
	McpListToolsCompleted(ResponseMcpListToolsCompletedEvent),
	#[serde(rename = "response.mcp_list_tools.failed")]
	McpListToolsFailed(ResponseMcpListToolsFailedEvent),
	#[serde(rename = "response.mcp_list_tools.in_progress")]
	McpListToolsInProgress(ResponseMcpListToolsInProgressEvent),
	#[serde(rename = "response.output_text.annotation.added")]
	OutputTextAnnotationAdded(ResponseOutputTextAnnotationAddedEvent),
	#[serde(rename = "response.reasoning.delta")]
	ReasoningDelta(ResponseReasoningDeltaEvent),
	#[serde(rename = "response.reasoning.done")]
	ReasoningDone(ResponseReasoningDoneEvent),
	#[serde(rename = "response.reasoning_summary.delta")]
	ReasoningSummaryDelta(ResponseReasoningSummaryDeltaEvent),
	#[serde(rename = "response.reasoning_summary.done")]
	ReasoningSummaryDone(ResponseReasoningSummaryDoneEvent),
	#[serde(rename = "error")]
	Error(ErrorEvent),
}

#[derive(Debug, Deserialize)]
pub struct EventBase {
	pub sequence_number: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseCreatedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: ResponseObject,
}

#[derive(Debug, Deserialize)]
pub struct ResponseInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: ResponseObject,
}

#[derive(Debug, Deserialize)]
pub struct ResponseCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: ResponseObject,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFailedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: ResponseObject,
}

#[derive(Debug, Deserialize)]
pub struct ResponseIncompleteEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: ResponseObject,
}

#[derive(Debug, Deserialize)]
pub struct ResponseQueuedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub response: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseOutputItemAddedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub output_index: u32,
	pub item: ResponseOutput,
}

#[derive(Debug, Deserialize)]
pub struct ResponseOutputItemDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub output_index: u32,
	pub item: ResponseOutput,
}

#[derive(Debug, Deserialize)]
pub struct ResponseContentPartAddedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub part: ResponseMessageOutputContent,
}

#[derive(Debug, Deserialize)]
pub struct ResponseContentPartDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub part: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseOutputTextDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub delta: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseOutputTextDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseRefusalDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub delta: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseRefusalDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub refusal: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub delta: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFileSearchCallInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFileSearchCallSearchingEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFileSearchCallCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWebSearchCallInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWebSearchCallSearchingEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWebSearchCallCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryPartAddedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub part: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryPartDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub part: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub delta: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryTextDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseImageGenerationCallCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseImageGenerationCallGeneratingEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseImageGenerationCallInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseImageGenerationCallPartialImageEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub partial_image_index: u32,
	pub partial_image_b64: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpCallArgumentsDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub delta: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpCallArgumentsDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub arguments: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpCallCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpCallFailedEvent {
	#[serde(flatten)]
	pub base: EventBase,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpCallInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpListToolsCompletedEvent {
	#[serde(flatten)]
	pub base: EventBase,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpListToolsFailedEvent {
	#[serde(flatten)]
	pub base: EventBase,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMcpListToolsInProgressEvent {
	#[serde(flatten)]
	pub base: EventBase,
}

#[derive(Debug, Deserialize)]
pub struct ResponseOutputTextAnnotationAddedEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub annotation_index: u32,
	pub annotation: Annotation,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub delta: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub content_index: u32,
	pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryDeltaEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub delta: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResponseReasoningSummaryDoneEvent {
	#[serde(flatten)]
	pub base: EventBase,
	pub item_id: String,
	pub output_index: u32,
	pub summary_index: u32,
	pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorEvent {
	#[serde(flatten)]
	pub error: ErrorBase,
	#[serde(flatten)]
	pub event: EventBase,
}
