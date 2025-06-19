//! OpenAI responses object types.
//!
//! <https://platform.openai.com/docs/api-reference/responses/object>

#![allow(missing_docs)]

// self
use super::r#type::*;
use crate::_prelude::*;

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseObject {
	pub background: Option<bool>,
	pub created_at: u64,
	pub error: Option<ResponseError>,
	pub id: String,
	pub incomplete_details: Option<IncompleteDetails>,
	pub instructions: Option<String>,
	pub max_output_tokens: Option<u32>,
	pub metadata: Value,
	pub model: Model,
	// Can be ignored.
	// pub object: String,
	pub output: Vec<ResponseOutput>,
	pub output_text: Option<String>,
	pub parallel_tool_calls: bool,
	pub previous_response_id: Option<String>,
	pub reasoning: Option<Reasoning>,
	pub service_tier: Option<ServiceTier>,
	pub status: ResponseStatus,
	pub temperature: Option<f32>,
	pub text: Text,
	pub tool_choice: ToolChoice,
	pub tools: Vec<Tool>,
	pub top_p: Option<f32>,
	pub truncation: Option<Truncation>,
	pub usage: Option<ResponseUsage>,
	pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseError {
	pub code: String,
	pub message: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IncompleteDetails {
	pub reason: String,
}

impl_deserializable_enum! {
	ResponseStatus {
		Completed => "completed",
		Failed => "failed",
		InProgress => "in_progress",
		Canceled => "canceled",
		Queued => "queued",
		Incomplete => "incomplete",
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutput {
	Message(ResponseOutputMessage),
	FileSearchCall(FileSearchCall),
	FunctionCall(FunctionCall),
	WebSearchCall(WebSearchCall),
	ComputerCall(ComputerCall),
	Reasoning(ReasoningItem),
	ImageGenerationCall(ImageGenerationCall),
	CodeInterpreterCall(CodeInterpreterCall),
	LocalShellCall(LocalShellCall),
	McpCall(McpCall),
	McpListTools(McpListTools),
	McpApprovalRequest(McpApprovalRequest),
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseUsage {
	pub input_tokens: u32,
	pub input_tokens_details: ResponseInputTokensDetails,
	pub output_tokens: u32,
	pub output_tokens_details: ResponseOutputTokensDetails,
	pub total_tokens: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseInputTokensDetails {
	pub cached_tokens: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseOutputTokensDetails {
	pub reasoning_tokens: u32,
}

#[test]
fn deserialization_should_work() {
	let resp = r#"{
	"background": true,
	"created_at": 1609459200,
	"error": {
		"code": "foo",
		"message": "foo"
	},
	"id": "foo",
	"incomplete_details": {
		"reason": "foo"
	},
	"instructions": "foo",
	"max_output_tokens": 2048,
	"metadata": {"foo": "bar"},
	"model": "gpt-4o",
	"output": [
		{
			"content": [
				{
					"annotations": [
						{
							"file_id": "foo",
							"index": 0,
							"type": "file_citation"
						}
					],
					"text": "foo",
					"type": "output_text"
				}
			],
			"role": "assistant",
			"type": "message",
			"id": "foo",
			"status": "completed"
		},
		{
			"id": "foo",
			"queries": ["foo", "bar"],
			"status": "in_progress",
			"type": "file_search_call",
			"results": [
				{
					"attributes": {"foo": "bar"},
					"file_id": "foo",
					"filename": "foo.bar",
					"score": 0.95,
					"text": "baz"
				}
			]
		},
		{
			"arguments": "{\"foo\":\"bar\"}",
			"call_id": "foo",
			"name": "foo",
			"type": "function_call",
			"id": "foo",
			"status": "completed"
		},
		{
			"id": "foo",
			"status": "completed",
			"type": "web_search_call"
		},
		{
			"id": "foo",
			"summary": [
				{
					"text": "foo",
					"type": "summary_text"
				}
			],
			"type": "reasoning",
			"encrypted_content": "foo",
			"status": "completed"
		},
		{
			"id": "foo",
			"result": "foo",
			"status": "completed",
			"type": "image_generation_call"
		},
		{
			"code": "foo('bar')",
			"id": "foo",
			"results": [
				{
					"logs": "foo",
					"type": "logs"
				}
			],
			"status": "completed",
			"type": "code_interpreter_call",
			"container_id": "foo"
		},
		{
			"action": {
				"command": ["foo", "bar"],
				"env": {"FOO": "bar"},
				"type": "exec",
				"timeout_ms": 5000,
				"user": "foo",
				"working_directory": "/foo"
			},
			"call_id": "foo",
			"id": "foo",
			"status": "completed",
			"type": "local_shell_call"
		},
		{
			"arguments": "{\"foo\":\"bar\"}",
			"id": "foo",
			"name": "foo",
			"server_label": "foo",
			"type": "mcp_call",
			"output": "{\"foo\":\"bar\"}"
		},
		{
			"id": "foo",
			"server_label": "foo",
			"tools": [
				{
					"input_schema": {
						"properties": {"foo": {"type": "string"}},
						"type": "object"
					},
					"name": "foo",
					"annotations": {"foo": "bar"},
					"description": "foo"
				}
			],
			"type": "mcp_list_tools"
		},
		{
			"arguments": "{\"foo\":\"bar\"}",
			"id": "foo",
			"name": "foo",
			"server_label": "foo",
			"type": "mcp_approval_request"
		}
	],
	"output_text": "foo",
	"parallel_tool_calls": true,
	"previous_response_id": "foo",
	"reasoning": {
		"effort": "high",
		"summary": "detailed"
	},
	"service_tier": "flex",
	"status": "completed",
	"temperature": 0.7,
	"text": {
		"format": {
			"name": "foo",
			"schema": {
				"type": "object",
				"properties": {"foo": {"type": "string"}},
				"required": ["foo"]
			},
			"type": "json_schema",
			"description": "foo",
			"strict": true
		}
	},
	"tool_choice": "auto",
	"tools": [
		{
			"name": "foo",
			"parameters": {
				"type": "object",
				"properties": {"foo": {"type": "string"}},
				"required": ["foo"]
			},
			"strict": true,
			"type": "function",
			"description": "foo"
		}
	],
	"top_p": 0.95,
	"truncation": "auto",
	"usage": {
		"input_tokens": 100,
		"input_tokens_details": {
			"cached_tokens": 50
		},
		"output_tokens": 200,
		"output_tokens_details": {
			"reasoning_tokens": 150
		},
		"total_tokens": 300
	},
	"user": "foo"
}"#;
	let deserialized = serde_json::from_str::<ResponseObject>(resp)
		.expect("comprehensive deserialization must succeed; qed");

	println!("{deserialized:?}");
}
