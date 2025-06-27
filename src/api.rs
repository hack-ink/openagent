//! OpenAI API

pub mod batch;
pub mod chat;
pub mod embedding;
pub mod file;
pub mod response;
pub mod r#type;

// self
use crate::_prelude::*;

/// JSON event handler for OpenAI API events.
pub struct ApiEventHandler<T>(PhantomData<T>);
impl<T> ApiEventHandler<T> {
	/// Create a new JSON event handler.
	pub fn new() -> Self {
		Self(PhantomData)
	}
}
impl<T> Default for ApiEventHandler<T> {
	fn default() -> Self {
		Self::new()
	}
}
impl<T> EventHandler for ApiEventHandler<T>
where
	T: Send + DeserializeOwned,
{
	type Event = T;

	fn handle_data(&self, data: String) -> Result<Self::Event> {
		Ok(serde_json::from_str(&data)?)
	}

	fn handle_unexpected(&self, unexpected: String) -> Result<()> {
		if let Ok(e) = serde_json::from_str::<ApiErrorWrapper>(&unexpected) {
			Err(Error::Api(e.error))
		} else {
			Err(Error::any(unexpected))
		}
	}
}
