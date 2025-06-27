//! OpenAI API Agent Kit

#![deny(clippy::all, missing_docs)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

// pub mod agent;
pub mod api;
pub mod error;
pub mod http;
// pub mod mcp;
// pub mod tool;
pub mod r#type;

pub mod prelude {
	#![allow(missing_docs)]

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

	pub(crate) type Map = serde_json::Map<String, Value>;
}
