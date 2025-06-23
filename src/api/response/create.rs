//! OpenAI responses create types.
//!
//! <https://platform.openai.com/docs/api-reference/responses/create>

#![allow(missing_docs)]

// self
use super::r#type::*;
use crate::_prelude::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ResponseRequest {
	pub input: Either<String, Vec<ResponseInput>>,
	pub model: Model,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<Include>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_output_tokens: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Map>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_response_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning: Option<Reasoning>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<ServiceTier>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<Text>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<ToolChoice>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<Tool>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation: Option<Truncation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ResponseInput {
	Message(ResponseMessage<Either<String, Vec<ResponseMessageInputContent>>>),
	Item(ResponseInputItem),
	ItemReference { id: String },
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseMessageInputContent {
	InputText {
		text: String,
	},
	InputImage {
		detail: ImageDetail,
		#[serde(skip_serializing_if = "Option::is_none")]
		file_id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		image_url: Option<String>,
	},
	InputFile {
		#[serde(skip_serializing_if = "Option::is_none")]
		file_data: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		file_id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		filename: Option<String>,
	},
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
	Message(Either<ResponseInputMessage, ResponseOutputMessage>),
	FileSearchCall(FileSearchCall),
	ComputerCall(ComputerCall),
	ComputerCallOutput {
		call_id: String,
		output: ComputerScreenshot,
		#[serde(skip_serializing_if = "Option::is_none")]
		acknowledged_safety_checks: Option<Vec<AcknowledgedSafetyCheck>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		status: Option<Status3>,
	},
	WebSearchCall(WebSearchCall),
	FunctionCall(FunctionCall),
	FunctionCallOutput {
		call_id: String,
		output: Value,
		#[serde(skip_serializing_if = "Option::is_none")]
		id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		status: Option<Status3>,
	},
	Reasoning(ReasoningItem),
	ImageGenerationCall(ImageGenerationCall),
	CodeInterpreterCall(CodeInterpreterCall),
	LocalShellCall(LocalShellCall),
	LocalShellCallOutput {
		id: String,
		output: Value,
		#[serde(skip_serializing_if = "Option::is_none")]
		status: Option<Status3>,
	},
	McpListTools(McpListTools),
	McpApprovalRequest(McpApprovalRequest),
	McpApprovalResponse {
		approval_request_id: String,
		approved: bool,
		#[serde(skip_serializing_if = "Option::is_none")]
		id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		reason: Option<String>,
	},
	McpCall(McpCall),
}

#[derive(Clone, Debug, Serialize)]
pub struct ComputerScreenshot {
	pub r#type: ConstComputerScreenshot,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
}

impl_const_str! {
	ComputerScreenshot => "computer_screenshot",
}

#[derive(Clone, Debug, Serialize)]
pub struct AcknowledgedSafetyCheck {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseInputMessage {
	#[serde(flatten)]
	pub message: ResponseMessage<Vec<ResponseMessageInputContent>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Status3>,
}

impl_serializable_enum! {
	Include {
		FileSearchCallResults => "file_search_call.results",
		MessageInputImageImageUrl => "message.input_image.image_url",
		ComputerCallOutputOutputImageUrl => "computer_call_output.output.image_url",
		ReasoningEncryptedContent => "reasoning.encrypted_content",
		CodeInterpreterCallOutputs => "code_interpreter_call.outputs",
	}
}

#[test]
fn serialization_should_work() {
	let req = ResponseRequest {
		input: Either::B(vec![
			ResponseInput::Message(ResponseMessage {
				content: Either::B(vec![
					ResponseMessageInputContent::InputText { text: "foo".into() },
					ResponseMessageInputContent::InputImage {
						detail: ImageDetail::High,
						file_id: Some("foo".into()),
						image_url: Some("https://foo.bar/baz.png".into()),
					},
					ResponseMessageInputContent::InputFile {
						file_data: Some("foo".into()),
						file_id: Some("foo".into()),
						filename: Some("foo.bar".into()),
					},
				]),
				role: Role::User,
			}),
			ResponseInput::ItemReference { id: "foo".into() },
			ResponseInput::Item(ResponseInputItem::Message(Either::A(ResponseInputMessage {
				message: ResponseMessage {
					content: vec![
						ResponseMessageInputContent::InputText { text: "foo".into() },
						ResponseMessageInputContent::InputImage {
							detail: ImageDetail::High,
							file_id: Some("foo".into()),
							image_url: Some("https://foo.bar/baz.png".into()),
						},
						ResponseMessageInputContent::InputFile {
							file_data: Some("foo".into()),
							file_id: Some("foo".into()),
							filename: Some("foo.bar".into()),
						},
					],
					role: Role::User,
				},
				status: Some(Status3::Completed),
			}))),
			ResponseInput::Item(ResponseInputItem::Message(Either::B(ResponseOutputMessage {
				message: ResponseMessage {
					content: vec![ResponseMessageOutputContent::OutputText {
						annotations: vec![Annotation::FileCitation {
							file_id: "foo".into(),
							index: 0,
						}],
						text: "foo".into(),

						logprobs: None,
					}],
					role: Role::Assistant,
				},
				id: "foo".into(),
				status: Status3::Completed,
			}))),
			ResponseInput::Item(ResponseInputItem::FileSearchCall(FileSearchCall {
				id: "foo".into(),
				queries: vec!["foo".into(), "bar".into()],
				status: FileSearchToolCallStatus::InProgress,
				results: Some(vec![FileSearchToolCallResult {
					attributes: Some(Map::from_iter([("foo".into(), "bar".into())])),
					file_id: Some("foo".into()),
					filename: Some("foo.bar".into()),
					score: Some(0.95),
					text: Some("foo".into()),
				}]),
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Click {
					button: Button::Left,

					coordinate: Coordinate { x: 100, y: 200 },
				},
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![PendingSafetyCheck {
					code: "foo".into(),
					id: "foo".into(),
					message: "foo".into(),
				}],
				status: Status3::InProgress,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Drag {
					path: vec![Coordinate { x: 100, y: 200 }, Coordinate { x: 300, y: 400 }],
				},
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Completed,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::DoubleClick {
					coordinate: Coordinate { x: 150, y: 250 },
				},
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Incomplete,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Keypress { keys: vec!["cmd".into(), "c".into()] },
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::InProgress,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Move { coordinate: Coordinate { x: 50, y: 75 } },
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Completed,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Screenshot,
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Completed,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Scroll {
					scroll_x: 100,
					scroll_y: -200,

					coordinate: Coordinate { x: 400, y: 300 },
				},
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::InProgress,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Type { text: "foo".into() },
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Completed,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCall(ComputerCall {
				action: ComputerToolCallAction::Wait,
				call_id: "foo".into(),
				id: "foo".into(),
				pending_safety_checks: vec![],
				status: Status3::Completed,
			})),
			ResponseInput::Item(ResponseInputItem::ComputerCallOutput {
				call_id: "foo".into(),
				output: ComputerScreenshot {
					r#type: Default::default(),
					file_id: Some("foo".into()),
					image_url: Some("https://foo.bar/baz.png".into()),
				},
				acknowledged_safety_checks: Some(vec![AcknowledgedSafetyCheck {
					id: "foo".into(),
					code: Some("foo".into()),
					message: Some("foo".into()),
				}]),
				id: Some("foo".into()),
				status: Some(Status3::Completed),
			}),
			ResponseInput::Item(ResponseInputItem::WebSearchCall(WebSearchCall {
				id: "foo".into(),
				status: "foo".into(),
			})),
			ResponseInput::Item(ResponseInputItem::FunctionCall(FunctionCall {
				arguments: serde_json::json!({"foo":"bar"}),
				call_id: "foo".into(),
				name: "foo".into(),
				id: Some("foo".into()),
				status: Some(Status3::Completed),
			})),
			ResponseInput::Item(ResponseInputItem::FunctionCallOutput {
				call_id: "foo".into(),
				output: serde_json::json!({"foo":"bar"}),
				id: Some("foo".into()),
				status: Some(Status3::Completed),
			}),
			ResponseInput::Item(ResponseInputItem::Reasoning(ReasoningItem {
				id: "foo".into(),
				summary: vec![SummaryText { text: "foo".into(), r#type: Default::default() }],
				encrypted_content: Some("foo".into()),
				status: Some(Status3::Completed),
			})),
			ResponseInput::Item(ResponseInputItem::ImageGenerationCall(ImageGenerationCall {
				id: "foo".into(),
				result: Some("foo".into()),
				status: "foo".into(),
			})),
			ResponseInput::Item(ResponseInputItem::CodeInterpreterCall(CodeInterpreterCall {
				code: "foo('bar')".into(),
				id: "foo".into(),
				results: vec![CodeInterpreterCallOutput::Logs { logs: "foo".into() }],
				status: "foo".into(),
				container_id: Some("foo".into()),
			})),
			ResponseInput::Item(ResponseInputItem::LocalShellCall(LocalShellCall {
				action: ShellAction {
					command: vec!["foo".into(), "bar".into()],
					env: serde_json::json!({"FOO":"bar"}),
					r#type: Default::default(),
					timeout_ms: Some(5000),
					user: Some("foo".into()),
					working_directory: Some("/foo".into()),
				},
				call_id: "foo".into(),
				id: "foo".into(),
				status: "running".into(),
			})),
			ResponseInput::Item(ResponseInputItem::LocalShellCallOutput {
				id: "foo".into(),
				output: serde_json::json!({"foo":"bar"}),
				status: Some(Status3::Completed),
			}),
			ResponseInput::Item(ResponseInputItem::McpListTools(McpListTools {
				id: "foo".into(),
				server_label: "foo".into(),
				tools: vec![ToolInfo {
					input_schema: serde_json::json!({"type":"object","properties":{"foo":{"type":"string"}}}),
					name: "foo".into(),
					annotations: Some(serde_json::json!({"foo":"bar"})),
					description: Some("foo".into()),
				}],
				error: None,
			})),
			ResponseInput::Item(ResponseInputItem::McpApprovalRequest(McpApprovalRequest {
				arguments: serde_json::json!({"foo":"bar"}),
				id: "foo".into(),
				name: "foo".into(),
				server_label: "foo".into(),
			})),
			ResponseInput::Item(ResponseInputItem::McpApprovalResponse {
				approval_request_id: "foo".into(),
				approved: true,
				id: Some("foo".into()),
				reason: Some("foo".into()),
			}),
			ResponseInput::Item(ResponseInputItem::McpCall(McpCall {
				arguments: serde_json::json!({"foo":"bar"}),
				id: "foo".into(),
				name: "foo".into(),
				server_label: "foo".into(),
				error: None,
				output: Some(r#"{"foo":"bar"}"#.into()),
			})),
		]),
		model: Model::Gpt4o,
		background: Some(true),
		include: Some(vec![
			Include::FileSearchCallResults,
			Include::MessageInputImageImageUrl,
			Include::ComputerCallOutputOutputImageUrl,
			Include::ReasoningEncryptedContent,
			Include::CodeInterpreterCallOutputs,
		]),
		instructions: Some("foo".into()),
		max_output_tokens: Some(2048),
		metadata: Some(Map::from_iter([("foo".into(), "bar".into())])),
		parallel_tool_calls: Some(true),
		previous_response_id: Some("foo".into()),
		reasoning: Some(Reasoning {
			effort: Some(ReasoningEffort::High),
			summary: Some(Summary::Detailed),
		}),
		service_tier: Some(ServiceTier::Flex),
		store: Some(true),
		stream: Some(false),
		temperature: Some(0.7),
		text: Some(Text {
			format: Some(ResponseTextFormat::JsonSchema {
				name: "foo".into(),
				schema: serde_json::json!({"type":"object","properties":{"foo":{"type":"string"}},"required":["bar"]}),
				description: Some("foo".into()),
				strict: Some(true),
			}),
		}),
		tool_choice: Some(ToolChoice::HostedTool { r#type: HostedTool::FileSearch }),
		tools: Some(vec![
			Tool::Function {
				name: "foo".into(),
				parameters: serde_json::json!({"type":"object","properties":{"foo":{"type":"string"}},"required":["bar"]}),
				strict: true,
				description: Some("foo".into()),
			},
			Tool::FileSearch {
				vector_store_ids: vec!["foo".into(), "bar".into()],
				filters: Some(FileSearchFilters::Eq(ComparisonFilter {
					key: "foo".into(),
					value: ComparisonValue::String("foo".into()),
				})),
				max_num_results: Some(20),
				ranking_options: Some(RankingOptions {
					ranker: Some("foo".into()),
					score_threshold: Some(0.8),
				}),
			},
			Tool::WebSearchPreview {
				search_context_size: Some(SearchContextSize::High),
				user_location: Some(Location {
					r#type: Default::default(),
					city: Some("foo".into()),
					country: Some("US".into()),
					region: Some("foo".into()),
					timezone: Some("foo".into()),
				}),
			},
			Tool::ComputerUsePreview {
				display_height: 1440,
				display_width: 2560,
				environment: "foo".into(),
			},
			Tool::Mcp {
				server_label: "foo".into(),
				server_url: "https://foo.bar/baz".into(),
				allowed_tools: Some(Either::A(vec!["foo".into(), "bar".into()])),
				headers: Some(serde_json::json!({"foo":"bar"})),
				require_approval: Some(Either::B(McpApprovalSetting::Always)),
			},
			Tool::CodeInterpreter {
				container: Either::B(CodeInterpreterContainer {
					r#type: Default::default(),
					file_ids: Some(vec!["foo".into(), "bar".into()]),
				}),
			},
			Tool::ImageGeneration {
				background: Some(ImageBackground::Transparent),
				input_image_mask: Some(InputImageMask {
					file_id: Some("foo".into()),
					image_url: Some("https://foo.bar/baz.png".into()),
				}),
				model: Some(Model::Custom {
					id: "foo".into(),
					name: "foo".into(),
					embedding: false,
					reasoning: false,
					function_calling: false,
				}),
				moderation: Some("strict".into()),
				output_compression: Some(90),
				output_format: Some(ImageFormat::Png),
				partial_images: Some(5),
				quality: Some(ImageQuality::High),
				size: Some(ImageSize::W1024H1536),
			},
			Tool::LocalShell,
		]),
		top_p: Some(0.95),
		truncation: Some(Truncation::Auto),
		user: Some("foo".into()),
	};
	let serialized = serde_json::to_string(&req).expect("serialization must succeed; qed");

	println!("{serialized}");
}
