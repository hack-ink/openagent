// crates.io
use serde::Serialize;
// self
use openagent::prelude::*;

fn main() {
	let req = ResponseRequest {
		input: vec![
			Either::A("Hello, how are you?".to_string()),
			Either::B(vec![
				Input::ResponseMessage(InputMessage::User(InputMessageContent {
					content: "What is the weather like today?".to_string(),
				})),
				Input::Item(InputItem::ResponseMessage(Either::A(InputMessage::Assistant(
					InputMessageContent { content: "The weather is sunny.".to_string() },
				)))),
				Input::Item(InputItem::ResponseMessage(Either::B(OutputMessage::Assistant(
					OutputMessageContent {
						id: "1".to_string(),
						status: "success".to_string(),
						content: "The weather is sunny.".to_string(),
					},
				)))),
				Input::Item(InputItem::FileSearchCall),
			]),
		],
	};
	let serialized = serde_json::to_string(&req).unwrap();

	println!("{serialized}");
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseRequest {
	pub input: Vec<Either<String, Vec<Input>>>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Input {
	ResponseMessage(InputMessage),
	Item(InputItem),
	// ItemReference(String),
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputItem {
	ResponseMessage(Either<InputMessage, OutputMessage>),
	FileSearchCall,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum InputMessage {
	User(InputMessageContent),
	Assistant(InputMessageContent),
	System(InputMessageContent),
	Developer(InputMessageContent),
}

#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContent {
	pub content: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum OutputMessage {
	Assistant(OutputMessageContent),
}

#[derive(Clone, Debug, Serialize)]
pub struct OutputMessageContent {
	pub id: String,
	pub status: String,
	pub content: String,
}
