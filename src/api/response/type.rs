//! OpenAI response API common types.

#![allow(missing_docs)]

// self
use crate::_prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMessage<T> {
	pub content: T,
	pub role: Role,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseOutputMessage {
	#[serde(flatten)]
	pub message: ResponseMessage<Vec<ResponseMessageOutputContent>>,
	pub id: String,
	pub status: Status3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseMessageOutputContent {
	OutputText {
		annotations: Vec<Annotation>,
		text: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		logprobs: Option<Vec<Logprobs>>,
	},
	Refusal(Refusal),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
	FileCitation {
		file_id: String,
		index: u32,
	},
	UrlCitation {
		end_index: u32,
		start_index: u32,
		title: String,
		url: String,
	},
	ContainerFileCitation {
		container_id: String,
		end_index: u32,
		file_id: String,
		start_index: u32,
	},
	FilePath {
		file_id: String,
		index: u32,
	},
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Refusal {
	pub refusal: String,
}

impl_serializable_deserializable_enum! {
	Status3 {
		InProgress => "in_progress",
		Completed => "completed",
		Incomplete => "incomplete",
	}
}
impl Status3 {
	pub fn in_progress(&self) -> bool {
		matches!(self, Self::InProgress)
	}

	pub fn completed(&self) -> bool {
		matches!(self, Self::Completed)
	}

	pub fn incomplete(&self) -> bool {
		matches!(self, Self::Incomplete)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSearchCall {
	pub id: String,
	pub queries: Vec<String>,
	pub status: FileSearchToolCallStatus,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<Vec<FileSearchToolCallResult>>,
}

impl_serializable_deserializable_enum! {
	FileSearchToolCallStatus {
		InProgress => "in_progress",
		Searching => "searching",
		Incomplete => "incomplete",
		Failed => "failed",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSearchToolCallResult {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Map>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputerCall {
	pub action: ComputerToolCallAction,
	pub call_id: String,
	pub id: String,
	pub pending_safety_checks: Vec<PendingSafetyCheck>,
	pub status: Status3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComputerToolCallAction {
	Click {
		button: Button,
		#[serde(flatten)]
		coordinate: Coordinate,
	},
	DoubleClick {
		#[serde(flatten)]
		coordinate: Coordinate,
	},
	Drag {
		path: Vec<Coordinate>,
	},
	Keypress {
		// TODO: Keycode.
		keys: Vec<String>,
	},
	Move {
		#[serde(flatten)]
		coordinate: Coordinate,
	},
	Screenshot,
	Scroll {
		scroll_x: i32,
		scroll_y: i32,
		#[serde(flatten)]
		coordinate: Coordinate,
	},
	Type {
		text: String,
	},
	Wait,
}

impl_serializable_deserializable_enum! {
	Button {
		Left => "left",
		Right => "right",
		Wheel => "wheel",
		Back => "back",
		Forward => "forward",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinate {
	pub x: u32,
	pub y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingSafetyCheck {
	pub code: String,
	pub id: String,
	pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebSearchCall {
	pub id: String,
	pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCall {
	pub arguments: Value,
	pub call_id: String,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Status3>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningItem {
	pub id: String,
	pub summary: Vec<SummaryText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encrypted_content: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Status3>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummaryText {
	pub text: String,
	pub r#type: ConstSummaryText,
}

impl_const_str! {
	SummaryText  => "summary_text",
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageGenerationCall {
	pub id: String,
	// This field requires explicit null serialization.
	pub result: Option<String>,
	pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeInterpreterCall {
	pub code: String,
	pub id: String,
	pub results: Vec<CodeInterpreterCallOutput>,
	pub status: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub container_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeInterpreterCallOutput {
	Logs { logs: String },
	Files { files: Vec<FileOutput> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileOutput {
	pub file_id: String,
	pub mime_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalShellCall {
	pub action: ShellAction,
	pub call_id: String,
	pub id: String,
	pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShellAction {
	pub command: Vec<String>,
	pub env: Value,
	pub r#type: ConstExec,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout_ms: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub working_directory: Option<String>,
}

impl_const_str! {
	Exec  => "exec",
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpListTools {
	pub id: String,
	pub server_label: String,
	pub tools: Vec<ToolInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolInfo {
	pub input_schema: Value,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpApprovalRequest {
	pub arguments: Value,
	pub id: String,
	pub name: String,
	pub server_label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpCall {
	pub arguments: Value,
	pub id: String,
	pub name: String,
	pub server_label: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reasoning {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub effort: Option<ReasoningEffort>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub summary: Option<Summary>,
}

impl_serializable_deserializable_enum! {
	Summary {
		Auto => "auto",
		Concise => "concise",
		Detailed => "detailed"
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Text {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<ResponseTextFormat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseTextFormat {
	Text,
	JsonSchema {
		name: String,
		schema: Value,
		#[serde(skip_serializing_if = "Option::is_none")]
		description: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		strict: Option<bool>,
	},
	JsonObject,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
	Mode(ToolChoiceMode),
	HostedTool { r#type: HostedTool },
	FunctionTool { name: String, r#type: ConstFunction },
}

impl_serializable_deserializable_enum! {
	ToolChoiceMode {
		None => "none",
		Auto => "auto",
		Required => "required",
	}
}

impl_serializable_deserializable_enum! {
	HostedTool {
		FileSearch => "file_search",
		WebSearchPreview => "web_search_preview",
		WebSearchPreview20250311 => "web_search_preview_2025_03_11",
		ComputerUsePreview => "computer_use_preview",
		CodeInterpreter => "code_interpreter",
		Mcp => "mcp",
		ImageGeneration => "image_generation",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
	Function {
		name: String,
		parameters: Value,
		strict: bool,
		#[serde(skip_serializing_if = "Option::is_none")]
		description: Option<String>,
	},
	FileSearch {
		vector_store_ids: Vec<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		filters: Option<FileSearchFilters>,
		#[serde(skip_serializing_if = "Option::is_none")]
		max_num_results: Option<u32>,
		#[serde(skip_serializing_if = "Option::is_none")]
		ranking_options: Option<RankingOptions>,
	},
	WebSearchPreview {
		#[serde(skip_serializing_if = "Option::is_none")]
		search_context_size: Option<SearchContextSize>,
		#[serde(skip_serializing_if = "Option::is_none")]
		user_location: Option<Location>,
	},
	ComputerUsePreview {
		display_height: u32,
		display_width: u32,
		environment: String,
	},
	Mcp {
		server_label: String,
		server_url: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		allowed_tools: Option<Either<Vec<String>, McpFilter>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		headers: Option<Value>,
		#[serde(skip_serializing_if = "Option::is_none")]
		require_approval: Option<Either<McpApprovalFilter, McpApprovalSetting>>,
	},
	CodeInterpreter {
		container: Either<String, CodeInterpreterContainer>,
	},
	ImageGeneration {
		#[serde(skip_serializing_if = "Option::is_none")]
		background: Option<ImageBackground>,
		#[serde(skip_serializing_if = "Option::is_none")]
		input_image_mask: Option<InputImageMask>,
		#[serde(skip_serializing_if = "Option::is_none")]
		model: Option<Model>,
		#[serde(skip_serializing_if = "Option::is_none")]
		moderation: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		output_compression: Option<u8>,
		#[serde(skip_serializing_if = "Option::is_none")]
		output_format: Option<ImageFormat>,
		#[serde(skip_serializing_if = "Option::is_none")]
		partial_images: Option<u8>,
		#[serde(skip_serializing_if = "Option::is_none")]
		quality: Option<ImageQuality>,
		#[serde(skip_serializing_if = "Option::is_none")]
		size: Option<ImageSize>,
	},
	LocalShell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileSearchFilters {
	Eq(ComparisonFilter),
	Ne(ComparisonFilter),
	Gt(ComparisonFilter),
	Gte(ComparisonFilter),
	Lt(ComparisonFilter),
	Lte(ComparisonFilter),
	And { filters: Vec<FileSearchFilters> },
	Or { filters: Vec<FileSearchFilters> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComparisonFilter {
	pub key: String,
	pub value: ComparisonValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonValue {
	String(String),
	Number(f64),
	Boolean(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score_threshold: Option<f32>,
}

impl_serializable_deserializable_enum! {
	SearchContextSize {
		Low => "low",
		Medium => "medium",
		High => "high",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
	pub r#type: ConstApproximate,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	// TODO: Country code.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timezone: Option<String>,
}

impl_const_str! {
	Approximate  => "approximate",
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpFilter {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpApprovalFilter {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub always: Option<McpFilter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub never: Option<McpFilter>,
}

impl_serializable_deserializable_enum! {
	McpApprovalSetting {
		Always => "always",
		Never => "never",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeInterpreterContainer {
	pub r#type: ConstAuto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

impl_const_str! {
	Auto  => "auto",
}

impl_serializable_deserializable_enum! {
	ImageBackground {
		Transparent => "transparent",
		Opaque => "opaque",
		Auto => "auto",
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputImageMask {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
}

impl_serializable_deserializable_enum! {
	ImageFormat {
		Png => "png",
		Webp => "webp",
		Jpeg => "jpeg",
	}
}

impl_serializable_deserializable_enum! {
	ImageQuality {
		Low => "low",
		Medium => "medium",
		High => "high",
		Auto => "auto",
	}
}

impl_serializable_deserializable_enum! {
	ImageSize {
		W1024H1024 => "1024x1024",
		W1024H1536 => "1024x1536",
		W1536H1024 => "1536x1024",
		Auto => "auto",
	}
}

impl_serializable_deserializable_enum! {
	Truncation {
		Auto => "auto",
		Disabled => "disabled",
	}
}
