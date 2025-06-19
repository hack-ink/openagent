//! OpenAI Agent Kit

// #![deny(clippy::all, missing_docs, unused_crate_dependencies)]

// pub mod agent;
pub mod api;
pub mod error;
pub mod http;
pub mod mcp;
pub mod tool;
pub mod r#type;

pub mod prelude {
	pub use crate::{
		api::{ApiEventHandler, batch::*, chat::*, embedding::*, file::*, response::*, r#type::*},
		http::*,
		r#type::*,
	};
}

mod util;

mod _prelude {
	pub use std::{
		borrow::Cow,
		fmt::{Display, Formatter, Result as FmtResult},
		future::Future,
		marker::PhantomData,
	};

	pub use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
	pub use serde_json::Value;

	pub(crate) use crate::{api::r#type::*, error::*, http::*, r#type::*, util::*};
}
