// std
use std::{
	collections::HashMap,
	sync::Arc,
	time::{Duration, Instant},
};
// crates.io
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
	sync::mpsc::{self, Sender},
	time,
};
use tokio_stream::wrappers::ReceiverStream;
// self
use crate::{
	_prelude::*,
	http::{Auth, Client, Sse},
	response::{Message, ResponseRequest, Role, StreamOptions},
	tool::*,
};

/// The main ReAct agent that coordinates reasoning and tool execution.
///
/// This agent implements the ReAct (Reasoning + Acting) pattern where the agent
/// alternates between reasoning about the problem and taking actions using tools.
#[derive(Clone)]
pub struct Agent {
	client: Client,
	options: AgentOptions,
	custom_instructions: Option<String>,
	tools: HashMap<String, Arc<dyn Tool>>,
}
impl Agent {
	/// Create a new [`AgentBuilder`] with authentication.
	pub fn builder(auth: Auth) -> AgentBuilder {
		AgentBuilder { auth, options: Default::default(), custom_instructions: Default::default() }
	}

	/// Register a single tool with the agent.
	pub fn register_tool<T>(&mut self, tool: T)
	where
		T: 'static + Tool,
	{
		let name = tool.name().to_string();

		tracing::info!("registering tool: {name}");

		self.tools.insert(name, Arc::new(tool));
	}

	/// Register multiple tools at once.
	pub fn register_tools<I, T>(&mut self, tools: I)
	where
		I: IntoIterator<Item = T>,
		T: 'static + Tool,
	{
		tools.into_iter().for_each(|tool| self.register_tool(tool));
	}

	/// Find a registered tool by name.
	pub fn find_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
		self.tools.get(name).cloned()
	}

	/// Get a list of all registered tool names.
	pub fn list_tools(&self) -> Vec<String> {
		self.tools.keys().cloned().collect()
	}

	/// Execute the ReAct loop with streaming output.
	pub async fn react_stream(&self, state: AgentState) -> impl Stream<Item = AgentEvent> {
		let (tx, rx) = mpsc::channel(32);
		// Spawn the agent execution in a separate task.
		let agent = self.clone();

		tokio::spawn(async move {
			if let Err(e) = run_agent_stream(agent, state, tx.clone()).await {
				let _ = tx.send(AgentEvent::err(e.to_string())).await;
			}
		});

		ReceiverStream::new(rx)
	}

	/// Generate a reasoning stream with a pre-built prompt.
	pub async fn reasoning_stream_with_prompt(
		&self,
		prompt: String,
	) -> Result<impl Stream<Item = String> + '_> {
		let system_message = Message { role: Role::Developer, content: self.system_prompt() };
		let user_message = Message { role: Role::User, content: prompt };
		let params = ResponseRequest {
			messages: vec![system_message, user_message],
			temperature: Some(self.options.temperature),
			max_completion_tokens: Some(self.options.max_completion_tokens),
			..Default::default()
		};
		let sse = self.completion_stream(params).await?;

		Ok(Box::pin(sse.filter_map(|event| async move {
			match event {
				Ok(data) =>
					if let Ok(json) = serde_json::from_str::<Value>(&data) {
						json.get("choices")?
							.get(0)?
							.get("delta")?
							.get("content")?
							.as_str()
							.map(|s| s.to_string())
					} else {
						None
					},
				Err(e) => {
					tracing::warn!("error in reasoning stream: {e}");

					None
				},
			}
		})))
	}

	/// Generate a stream of reasoning tokens for a given state.
	pub async fn reasoning_stream<'a>(
		&'a self,
		state: &'a AgentState,
	) -> Result<impl Stream<Item = String> + 'a> {
		let prompt = self.build_prompt(state);
		let system_message = Message { role: Role::Developer, content: self.system_prompt() };
		let user_message = Message { role: Role::User, content: prompt };
		let params = ResponseRequest {
			messages: vec![system_message, user_message],
			temperature: Some(self.options.temperature),
			max_completion_tokens: Some(self.options.max_completion_tokens),
			..Default::default()
		};
		let sse = self.completion_stream(params).await?;

		Ok(Box::pin(sse.filter_map(|event| async move {
			match event {
				Ok(data) =>
					if let Ok(json) = serde_json::from_str::<Value>(&data) {
						json.get("choices")?
							.get(0)?
							.get("delta")?
							.get("content")?
							.as_str()
							.map(|s| s.to_string())
					} else {
						None
					},
				Err(e) => {
					tracing::warn!("error in reasoning stream: {e}");

					None
				},
			}
		})))
	}

	/// Build the prompt for the current reasoning step.
	fn build_prompt(&self, state: &AgentState) -> String {
		let mut prompt = format!("Question: {}\n\n", state.input);

		// Add conversation history.
		for (i, step) in state.reasoning_steps.iter().enumerate() {
			prompt.push_str(&format!("Thought {}: {step}\n", i + 1));

			// Add corresponding tool call if it exists.
			let Some(ToolCallResult { tool_call: ToolCall { name, args }, outcome }) =
				state.tool_calls.get(i)
			else {
				continue;
			};

			prompt.push_str(&format!("Action: {name}\n"));
			prompt.push_str(&format!("Action Input: {args}\n"));

			// Format observation based on tool call outcome.
			match &outcome {
				ToolCallOutcome::Success { result } =>
					prompt.push_str(&format!("Observation: {result}\n")),
				ToolCallOutcome::Error { message } => {
					prompt.push_str(&format!("Error: {message}\n"));
				},
			}

			prompt.push('\n');
		}

		// Add next thought prompt.
		let step_num = state.reasoning_steps.len() + 1;

		prompt.push_str(&format!("Thought {step_num}: "));

		prompt
	}

	/// Get the system prompt for the agent.
	///
	/// This method constructs a system prompt optimized for modern LLM capabilities
	/// including structured tool calling when available.
	fn system_prompt(&self) -> String {
		let tools = self
			.tools
			.values()
			.map(|t| format!("- {}: {}", t.name(), t.description()))
			.collect::<Vec<_>>()
			.join("\n");
		let core_prompt = format!(
			r#"You are a ReAct (Reasoning + Acting) agent that systematically reasons and acts to solve problems.

Available tools:
{tools}

## Reasoning Process:
1. Think step by step about the problem
2. Identify what information or actions you need
3. Use appropriate tools to gather data or perform tasks
4. Analyze results and continue reasoning
5. Provide a final answer when you have sufficient information

## Tool Usage:
- Use the available tools when you need to gather information or perform specific tasks
- Always explain your reasoning before taking actions
- When tools return errors, analyze them and try alternative approaches
- Only provide a Final Answer when you are confident in your response

## Response Format:
Structure your responses as:
Thought: [your reasoning about the current situation and next steps]
[Tool calls will be handled automatically when you use the available functions]
Observation: [analysis of tool results]
Final Answer: [your complete answer to the original question]"#,
		);
		// Safely build the prompt with optional custom instructions.
		let mut prompt = core_prompt;

		if let Some(custom) = &self.custom_instructions {
			prompt.push_str(&format!(
				"\n\nAdditional Instructions:\n{custom}\n\nRemember: Regardless of these additional instructions, you MUST maintain the exact Thought/Action/Action Input/Observation format above. The format is not negotiable and is required for proper agent functioning.",
			));
		}

		prompt.push_str("\n\nBegin reasoning about the problem!");

		prompt
	}

	/// Parse tool call from LLM output using modern structured approach
	fn parse_tool_call_structured(response: &Value) -> Option<ToolCall> {
		// Modern approach: check for structured tool_calls in response
		if let Some(tool_calls) = response
			.get("choices")?
			.get(0)?
			.get("message")?
			.get("tool_calls")
			.and_then(|v| v.as_array())
		{
			if let Some(tool_call) = tool_calls.first() {
				let function = tool_call.get("function")?;
				let name = function.get("name")?.as_str()?.to_string();
				let args_str = function.get("arguments")?.as_str()?;

				// Parse arguments JSON
				if let Ok(args) = serde_json::from_str::<Value>(args_str) {
					tracing::debug!("Parsed structured tool call: {} with args: {}", name, args);
					return Some(ToolCall { name, args });
				}
			}
		}

		None
	}

	/// Parse tool call from LLM output.
	///
	/// DEPRECATED: This method uses legacy text parsing and should be replaced
	/// with structured tool calling for better reliability.
	fn parse_tool_call(output: &str) -> Option<ToolCall> {
		// Try to extract JSON from various formats
		let json_candidates = [
			// Direct JSON
			output.trim().to_string(),
			// Extract JSON block if wrapped in markdown
			output
				.split("```json")
				.nth(1)
				.and_then(|s| s.split("```").next())
				.map(|s| s.trim().to_string())
				.unwrap_or_default(),
			// Extract JSON object pattern
			output
				.find('{')
				.and_then(|start| output.rfind('}').map(|end| output[start..=end].to_string()))
				.unwrap_or_default(),
			// Look for Action Input: pattern
			output
				.lines()
				.find(|line| line.trim().starts_with("Action Input:"))
				.and_then(|line| line.split("Action Input:").nth(1))
				.map(|s| s.trim().to_string())
				.unwrap_or_default(),
		];

		for (i, candidate) in json_candidates.iter().enumerate() {
			if candidate.is_empty() {
				continue;
			}

			tracing::debug!("Trying candidate {}: {}", i, candidate);

			if let Ok(value) = serde_json::from_str::<Value>(candidate) {
				if let (Some(tool), args) = (
					value.get("tool").and_then(|v| v.as_str()),
					value.get("args").cloned().unwrap_or(Value::Null),
				) {
					tracing::debug!("Successfully parsed tool call: {} with args: {}", tool, args);
					return Some(ToolCall { name: tool.to_string(), args });
				}
			}
		}

		tracing::debug!("Failed to parse tool call from output");

		None
	}

	/// Execute a tool call with timeout protection.
	async fn call_tool_with_timeout(
		&self,
		tx: &Sender<AgentEvent>,
		tool_req: ToolCall,
	) -> Result<ToolCallResult> {
		time::timeout(self.options.timeout, self.call_tool(tx, tool_req.clone())).await.map_err(
			|_| {
				let e = Error::Timeout(self.options.timeout);

				tracing::error!("{e}");

				e
			},
		)?
	}

	/// Execute a tool call
	///
	/// # Arguments
	/// * `tx` - Channel sender for agent events
	/// * `tool_req` - Tool call request
	///
	/// # Returns
	/// * `Result<ToolCallResult>` - Tool execution result
	async fn call_tool(
		&self,
		tx: &Sender<AgentEvent>,
		tool_req: ToolCall,
	) -> Result<ToolCallResult> {
		let ToolCall { name, args } = &tool_req;

		tracing::debug!("calling tool '{name}' with args: {args}");

		// Locate the tool or immediately propagate an error.
		let Some(tool) = self.find_tool(name) else {
			let e = ToolError::Unknown(name.to_owned());

			tracing::error!("{e}");

			let _ = tx.send(AgentEvent::err(e.to_string())).await;

			Err(e)?
		};

		// Prefer streaming path if supported
		if tool.supports_stream() {
			tracing::debug!("Using streaming execution for tool '{}'", name);
			match tool.call_stream(args.clone()).await {
				Ok(mut stream) => {
					let mut acc = String::new();

					while let Some(chunk) = stream.next().await {
						let _ = tx
							.send(AgentEvent::ToolResult {
								name: name.to_string(),
								result: Value::String(chunk.clone()),
								is_streaming: Some(true),
							})
							.await;

						acc.push_str(&chunk);
					}

					tracing::debug!("tool '{name}' streaming completed");

					return Ok(ToolCallResult::success(
						name.to_string(),
						args.clone(),
						Value::String(acc),
					));
				},
				Err(e) => {
					tracing::error!("{e}");

					let _ = tx.send(AgentEvent::err(e.to_string())).await;
					// Fall through to sync path
				},
			}
		}

		// Synchronous fallback
		tracing::debug!("Using synchronous execution for tool '{}'", name);
		match tool.call(args.clone()).await {
			Ok(result) => {
				tracing::debug!("Tool '{}' executed successfully", name);
				let _ = tx
					.send(AgentEvent::ToolResult {
						name: name.to_string(),
						result: result.clone(),
						is_streaming: Some(false),
					})
					.await;

				Ok(ToolCallResult::success(name.to_string(), args.clone(), result))
			},
			Err(err) => {
				tracing::error!("Tool '{}' execution failed: {}", name, err);
				Err(err)
			},
		}
	}

	/// Send a completion request to the LLM
	///
	/// # Arguments
	/// * `params` - Completion parameters
	///
	/// # Returns
	/// * `Result<String>` - LLM response
	async fn completion<P>(&self, params: P) -> Result<String>
	where
		P: Into<ResponseRequest>,
	{
		self.client.post(params.into()).await
	}

	/// Create a streaming completion request to the LLM
	///
	/// # Arguments
	/// * `params` - Completion parameters
	///
	/// # Returns
	/// * `Result<Sse>` - Server-sent events stream
	async fn completion_stream<P>(&self, params: P) -> Result<Sse>
	where
		P: Into<ResponseRequest>,
	{
		let mut params = params.into();
		params.stream = Some(true);
		params.stream_options = Some(StreamOptions { include_usage: true });

		self.client.sse_post(params).await
	}

	/// Validate that the agent's system prompt contains required ReAct format elements
	///
	/// This method helps detect potential issues with custom system prompts that might
	/// break the agent's parsing logic.
	///
	/// # Returns
	/// * `Result<()>` - Ok if the prompt appears valid, Err with details if issues found
	pub fn validate_system_prompt(&self) -> Result<()> {
		let prompt = self.system_prompt();
		let mut issues = Vec::new();

		// Check for required format keywords
		let required_keywords =
			["Thought:", "Action:", "Action Input:", "Observation:", "Final Answer:"];

		for keyword in &required_keywords {
			if !prompt.contains(keyword) {
				issues.push(format!("Missing required format keyword: '{}'", keyword));
			}
		}

		// Check for tool call JSON format requirement
		if !prompt.contains(r#"{"tool":"#) && !prompt.contains(r#"{"tool": "#) {
			issues.push("Missing required tool call JSON format specification".to_string());
		}

		// Check if tools are mentioned in the prompt
		if self.tools.is_empty() {
			issues.push("No tools available to the agent".to_string());
		} else {
			let tools_mentioned = self.tools.keys().any(|tool_name| prompt.contains(tool_name));
			if !tools_mentioned && !prompt.contains("Available tools:") {
				issues.push("Tools may not be properly described in the prompt".to_string());
			}
		}

		if issues.is_empty() {
			Ok(())
		} else {
			let e = format!(
				"System prompt validation failed with {} issues:\n{}",
				issues.len(),
				issues.join("\n- ")
			);

			Err(Error::any(e))
		}
	}

	/// Get tool definitions in OpenAI Function Calling format
	///
	/// This method generates the tools array for modern LLM APIs that support
	/// native function calling, eliminating the need for text parsing.
	pub fn get_tool_definitions(&self) -> Vec<Value> {
		self.tools
			.values()
			.map(|tool| {
				serde_json::json!({
					"type": "function",
					"function": {
						"name": tool.name(),
						"description": tool.description(),
						"parameters": tool.schema()
					}
				})
			})
			.collect()
	}

	/// Enhanced completion with structured tool calling support
	///
	/// This method attempts to use modern function calling APIs when available,
	/// falling back to text parsing for compatibility.
	async fn completion_with_tools<P>(&self, params: P) -> Result<(String, Vec<ToolCall>)>
	where
		P: Into<ResponseRequest>,
	{
		let mut params = params.into();

		// Add tools if available and supported
		if !self.tools.is_empty() {
			params.tools = Some(self.get_tool_definitions());
			params.tool_choice = Some(serde_json::json!("auto"));
		}

		let response_text = self.client.post(params).await?;

		// Try to parse as JSON response first (modern API)
		if let Ok(response_json) = serde_json::from_str::<Value>(&response_text) {
			if let Some(tool_call) = Self::parse_tool_call_structured(&response_json) {
				return Ok((response_text, vec![tool_call]));
			}
		}

		// Fallback to legacy text parsing
		let tool_calls =
			Self::parse_tool_call(&response_text).map(|tc| vec![tc]).unwrap_or_default();

		Ok((response_text, tool_calls))
	}
}

/// Builder for creating and configuring an Agent
///
/// Provides a fluent interface for setting up an agent with custom options.
pub struct AgentBuilder {
	pub auth: Auth,
	pub options: AgentOptions,
	pub custom_instructions: Option<String>,
}

impl AgentBuilder {
	/// Set the maximum number of reasoning steps
	///
	/// # Arguments
	/// * `steps` - Maximum number of steps (default: 10)
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn max_steps(mut self, steps: usize) -> Self {
		self.options.max_steps = steps;
		self
	}

	/// Set the timeout for individual tool executions
	///
	/// # Arguments
	/// * `duration` - Timeout duration (default: 300 seconds)
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn timeout(mut self, duration: Duration) -> Self {
		self.options.timeout = duration;
		self
	}

	/// Set the temperature for LLM responses
	///
	/// # Arguments
	/// * `temp` - Temperature value between 0.0 and 2.0 (default: 0.7)
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn temperature(mut self, temp: f32) -> Self {
		self.options.temperature = temp.clamp(0.0, 2.0);
		self
	}

	/// Set the maximum completion tokens for LLM responses
	///
	/// # Arguments
	/// * `tokens` - Maximum tokens (default: 4000)
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn max_completion_tokens(mut self, tokens: u32) -> Self {
		self.options.max_completion_tokens = tokens;
		self
	}

	/// Set custom instructions to add to the agent's reasoning context
	///
	/// These instructions will be added to the default system prompt without
	/// breaking the core ReAct format requirements.
	///
	/// # Arguments
	/// * `instructions` - Additional instructions for the agent
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn custom_instructions(mut self, instructions: String) -> Self {
		self.custom_instructions = Some(instructions);
		self
	}

	/// Enable reasoning effort for compatible models
	///
	/// # Arguments
	/// * `enabled` - Whether to enable reasoning effort
	///
	/// # Returns
	/// * `Self` - Builder for method chaining
	pub fn reasoning_effort(mut self, enabled: bool) -> Self {
		self.options.reasoning_effort = enabled;
		self
	}

	/// Build the [`Agent`] instance with the configured options.
	///
	/// # Arguments
	/// * `client` - HTTP client for API communication
	///
	/// # Returns
	/// * `Agent` - Configured agent instance
	pub fn build(self, client: Client) -> Agent {
		Agent {
			client,
			options: self.options,
			tools: HashMap::new(),
			custom_instructions: self.custom_instructions,
		}
	}
}

/// Represents the state of an agent during execution.
///
/// Contains all the information about the agent's reasoning process,
/// tool calls, and accumulated knowledge.
#[derive(Clone, Debug)]
pub struct AgentState {
	/// The original user input/question.
	pub input: String,
	/// Key-value storage for agent memory.
	pub memory: HashMap<String, String>,
	/// List of reasoning steps taken by the agent.
	pub reasoning_steps: Vec<String>,
	/// Results of tool calls made by the agent.
	pub tool_calls: Vec<ToolCallResult>,
	/// Execution metadata and statistics.
	pub metadata: AgentMetadata,
}

impl AgentState {
	/// Create a new agent state with the given input
	///
	/// # Arguments
	/// * `input` - The user's question or task description
	///
	/// # Returns
	/// * `Self` - New agent state
	pub fn new(input: String) -> Self {
		tracing::info!("Creating new agent state for input: {}", input);
		Self {
			input,
			reasoning_steps: Vec::new(),
			tool_calls: Vec::new(),
			memory: HashMap::new(),
			metadata: AgentMetadata::new(),
		}
	}

	/// Add a reasoning step to the agent's history
	///
	/// # Arguments
	/// * `reasoning` - The reasoning text from the LLM
	pub fn add_step(&mut self, reasoning: String) {
		tracing::debug!("Adding reasoning step: {}", reasoning);
		self.reasoning_steps.push(reasoning);
		self.metadata.total_steps += 1;
	}

	/// Add a tool call result to the agent's history
	///
	/// # Arguments
	/// * `result` - The result of a tool execution
	pub fn add_tool_call(&mut self, result: ToolCallResult) {
		tracing::debug!("Adding tool call result: {:?}", result);
		self.tool_calls.push(result);
		self.metadata.tool_calls_count += 1;
	}

	/// Store a key-value pair in the agent's memory
	///
	/// # Arguments
	/// * `key` - Memory key
	/// * `value` - Memory value
	pub fn remember(&mut self, key: String, value: String) {
		tracing::debug!("Storing in memory: {} = {}", key, value);
		self.memory.insert(key, value);
	}

	/// Retrieve a value from the agent's memory
	///
	/// # Arguments
	/// * `key` - Memory key
	///
	/// # Returns
	/// * `Option<&String>` - The stored value if found
	pub fn recall(&self, key: &str) -> Option<&String> {
		self.memory.get(key)
	}

	/// Get the current step number
	pub fn current_step(&self) -> usize {
		self.reasoning_steps.len()
	}

	/// Get the total number of steps (reasoning + tool calls)
	pub fn total_steps(&self) -> usize {
		self.metadata.total_steps
	}

	/// Check if the agent has completed its task
	pub fn is_complete(&self) -> bool {
		self.reasoning_steps.iter().any(|step| step.to_lowercase().contains("final answer:"))
	}
}

/// Metadata tracking agent execution statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AgentMetadata {
	/// Total number of reasoning steps taken
	pub total_steps: usize,
	/// Number of tool calls made
	pub tool_calls_count: usize,
	/// When execution started (not serialized)
	#[serde(skip)]
	pub start_time: Option<Instant>,
	/// When execution completed (not serialized)
	#[serde(skip)]
	pub end_time: Option<Instant>,
	/// Total execution duration in milliseconds
	pub duration_ms: Option<u64>,
}

impl AgentMetadata {
	/// Create new metadata with current timestamp
	pub fn new() -> Self {
		Self { start_time: Some(Instant::now()), ..Default::default() }
	}

	/// Mark execution as complete and calculate duration
	pub fn complete(&mut self) {
		let now = Instant::now();
		self.end_time = Some(now);
		if let Some(start) = self.start_time {
			let duration = now.duration_since(start);
			self.duration_ms = Some(duration.as_millis() as u64);
		}
	}

	/// Get execution duration if available
	pub fn get_duration(&self) -> Option<Duration> {
		if let Some(ms) = self.duration_ms {
			Some(Duration::from_millis(ms))
		} else if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
			Some(end.duration_since(start))
		} else {
			self.start_time.map(|start| Instant::now().duration_since(start))
		}
	}
}

/// Events emitted during agent execution
///
/// These events provide real-time updates about the agent's reasoning process,
/// tool executions, and final results.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AgentEvent {
	/// Individual reasoning token from the LLM
	#[serde(rename = "reasoningToken")]
	ReasoningToken { content: String },

	/// Complete reasoning step finished
	#[serde(rename = "reasoningStepDone")]
	ReasoningStepDone { content: String },

	/// Tool call initiated
	#[serde(rename = "toolCall")]
	ToolCall { name: String, args: Value },

	/// Tool execution result (can be streamed)
	#[serde(rename = "toolResult")]
	ToolResult {
		name: String,
		result: Value,
		#[serde(skip_serializing_if = "Option::is_none")]
		is_streaming: Option<bool>,
	},

	/// Agent's final answer to the question
	#[serde(rename = "finalAnswer")]
	FinalAnswer { content: String },

	/// Error occurred during execution
	#[serde(rename = "error")]
	Error { message: String },

	/// Execution metadata update
	#[serde(rename = "metadata")]
	Metadata {
		#[serde(flatten)]
		data: AgentMetadata,
	},

	/// Agent started execution
	#[serde(rename = "started")]
	Started { max_steps: usize, tools: Vec<String> },

	/// Agent completed execution
	#[serde(rename = "completed")]
	Completed { success: bool, total_steps: usize, duration: Option<Duration> },
}

impl AgentEvent {
	/// Create an error event.
	pub fn err(message: impl Into<String>) -> Self {
		Self::Error { message: message.into() }
	}

	/// Create a ReasoningToken event
	pub fn reasoning_token(content: impl Into<String>) -> Self {
		Self::ReasoningToken { content: content.into() }
	}

	/// Create a Started event
	pub fn started(max_steps: usize, tools: Vec<String>) -> Self {
		Self::Started { max_steps, tools }
	}

	/// Create a Completed event
	pub fn completed(success: bool, total_steps: usize, duration: Option<Duration>) -> Self {
		Self::Completed { success, total_steps, duration }
	}
}

/// Options for configuring the agent's behavior.
#[derive(Clone, Debug)]
pub struct AgentOptions {
	pub max_steps: usize,
	pub timeout: Duration,
	pub temperature: f32,
	pub max_completion_tokens: u32,
	pub reasoning_effort: bool,
}

impl Default for AgentOptions {
	fn default() -> Self {
		Self {
			max_steps: 10,
			timeout: Duration::from_secs(300),
			temperature: 0.7,
			max_completion_tokens: 4000,
			reasoning_effort: false,
		}
	}
}

/// Main agent execution loop with streaming updates
///
/// This function implements the ReAct pattern by alternating between reasoning
/// and action steps until a final answer is reached or limits are exceeded.
///
/// # Arguments
/// * `agent` - The agent instance
/// * `mut state` - Mutable agent state
/// * `tx` - Channel sender for streaming events
///
/// # Returns
/// * `Result<()>` - Success or error
async fn run_agent_stream(
	agent: Agent,
	mut state: AgentState,
	tx: Sender<AgentEvent>,
) -> Result<()> {
	tracing::info!("Starting agent execution for input: {}", state.input);

	// Send startup event
	let _ = tx.send(AgentEvent::started(agent.options.max_steps, agent.list_tools())).await;

	for step in 0..agent.options.max_steps {
		tracing::debug!("Starting step {} of {}", step + 1, agent.options.max_steps);

		// Check if we already have a final answer.
		if state.is_complete() {
			if let Some(answer) = extract_final_answer(state.reasoning_steps.last().unwrap()) {
				tracing::info!("Agent found final answer: {}", answer);
				let _ = tx.send(AgentEvent::FinalAnswer { content: answer }).await;
				let _ = tx
					.send(AgentEvent::completed(
						true,
						state.total_steps(),
						state.metadata.get_duration(),
					))
					.await;
				return Ok(());
			}
		}

		// Build prompt first to avoid borrow conflicts.
		let prompt = agent.build_prompt(&state);
		// Generate reasoning using prompt (no state reference needed).
		let reasoning_res = agent.reasoning_stream_with_prompt(prompt).await;

		match reasoning_res {
			Ok(mut stream) => {
				let mut full_reasoning = String::new();
				let mut token_buffer = Vec::new();

				// Collect reasoning tokens
				while let Some(token) = stream.next().await {
					token_buffer.push(token.clone());
					full_reasoning.push_str(&token);

					// Batch send tokens for better performance
					if token_buffer.len() >= 5 {
						for chunk in token_buffer.drain(..) {
							let _ = tx.send(AgentEvent::reasoning_token(chunk)).await;
						}
					}
				}

				// Send remaining tokens
				for chunk in token_buffer {
					let _ = tx.send(AgentEvent::reasoning_token(chunk)).await;
				}

				if full_reasoning.trim().is_empty() {
					tracing::warn!("Empty reasoning generated at step {}", step + 1);
					continue;
				}

				tracing::debug!("generated reasoning: {full_reasoning}");

				let _ = tx
					.send(AgentEvent::ReasoningStepDone { content: full_reasoning.clone() })
					.await;

				state.add_step(full_reasoning.clone());

				// Check for final answer in the reasoning
				if full_reasoning.to_lowercase().contains("final answer:") {
					if let Some(answer) = extract_final_answer(&full_reasoning) {
						tracing::info!("Agent provided final answer: {}", answer);
						let _ = tx.send(AgentEvent::FinalAnswer { content: answer }).await;

						state.metadata.complete();
						let _ = tx
							.send(AgentEvent::completed(
								true,
								state.total_steps(),
								state.metadata.get_duration(),
							))
							.await;
						return Ok(());
					}
				}

				// Parse for tool call
				if let Some(tool_req) = Agent::parse_tool_call(&full_reasoning) {
					tracing::info!(
						"Parsed tool call: {} with args: {}",
						tool_req.name,
						tool_req.args
					);

					let _ = tx
						.send(AgentEvent::ToolCall {
							name: tool_req.name.clone(),
							args: tool_req.args.clone(),
						})
						.await;

					match agent.call_tool_with_timeout(&tx, tool_req.clone()).await {
						Ok(result) => {
							tracing::info!("tool call successful: {}", result.tool_call.name);

							state.add_tool_call(result);
						},
						Err(e) => {
							let _ = tx.send(AgentEvent::err(e.to_string())).await;

							// Add error as observation for the agent to learn from
							state.add_tool_call(ToolCallResult::err(
								tool_req.name.clone(),
								tool_req.args.clone(),
								e.to_string(),
							));
						},
					}
				}
			},
			Err(e) => {
				tracing::error!("reasoning failed at step {}: {e}", step + 1);

				let _ = tx.send(AgentEvent::err(e.to_string())).await;

				state.metadata.complete();

				let _ = tx
					.send(AgentEvent::completed(
						false,
						state.total_steps(),
						state.metadata.get_duration(),
					))
					.await;

				return Err(e);
			},
		}

		// Send metadata update
		let _ = tx.send(AgentEvent::Metadata { data: state.metadata.clone() }).await;
	}

	// Max steps reached.
	tracing::warn!(
		"Agent reached maximum steps ({}) without final answer",
		agent.options.max_steps
	);

	state.metadata.complete();

	// TODO raise error.
	let e = AgentError::MaxStepsExceeded(agent.options.max_steps);
	let _ = tx.send(AgentEvent::err(e.to_string())).await;
	let _ = tx
		.send(AgentEvent::completed(false, state.total_steps(), state.metadata.get_duration()))
		.await;

	Ok(())
}

/// Extract final answer from reasoning text
///
/// # Arguments
/// * `text` - The reasoning text to search
///
/// # Returns
/// * `Option<String>` - Extracted final answer if found
fn extract_final_answer(text: &str) -> Option<String> {
	let lower_text = text.to_lowercase();

	// Look for "final answer:" pattern
	if let Some(pos) = lower_text.find("final answer:") {
		let after_marker = &text[pos + "final answer:".len()..];
		let answer = after_marker.trim();

		if !answer.is_empty() {
			return Some(answer.to_string());
		}
	}

	None
}
