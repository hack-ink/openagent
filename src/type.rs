//! OpenAI API General Types

// std
use std::sync::LazyLock;
// crates.io
use regex::Regex;
// self
use crate::_prelude::*;

/// Regex pattern for removing date suffixes from model identifiers
static RE_DATE_SUFFIX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"-\d{4}-\d{2}-\d{2}$").unwrap());

/// Represents different AI model types with their capabilities and identifiers
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Model {
	/// OpenAI's GPT-4o model for general conversation and reasoning
	#[default]
	Gpt4o,
	/// OpenAI's GPT-4o Mini model, a smaller version of GPT-4o
	Gpt4oMini,
	/// OpenAI's small text embedding model for vector representations
	TextEmbedding3Small,
	/// OpenAI's large text embedding model for higher quality vectors
	TextEmbedding3Large,
	/// OpenAI's legacy Ada text embedding model
	TextEmbeddingAda002,
	/// A custom model with user-defined capabilities and properties
	Custom {
		/// Unique identifier for the custom model
		id: Cow<'static, str>,
		/// Human-readable name for the custom model
		name: Cow<'static, str>,
		/// Whether this model supports text embedding operations
		embedding: bool,
		/// Whether this model supports reasoning capabilities
		reasoning: bool,
		/// Whether this model supports function calling
		function_calling: bool,
	},
	/// An unrecognized model identified only by its string ID
	Unknown(String),
}
impl Model {
	/// Creates a Model instance from a string identifier
	pub fn from_id(id: &str) -> Self {
		match id {
			"gpt-4o" => Self::Gpt4o,
			"gpt-4o-mini" => Self::Gpt4oMini,
			"text-embedding-3-small" => Self::TextEmbedding3Small,
			"text-embedding-3-large" => Self::TextEmbedding3Large,
			"text-embedding-ada-002" => Self::TextEmbeddingAda002,
			_ => Self::Unknown(id.to_owned()),
		}
	}

	/// Returns the unique identifier string for this model
	pub fn id(&self) -> Cow<'static, str> {
		match self {
			Self::Gpt4o => Cow::Borrowed("gpt-4o"),
			Self::Gpt4oMini => Cow::Borrowed("gpt-4o-mini"),
			Self::TextEmbedding3Small => Cow::Borrowed("text-embedding-3-small"),
			Self::TextEmbedding3Large => Cow::Borrowed("text-embedding-3-large"),
			Self::TextEmbeddingAda002 => Cow::Borrowed("text-embedding-ada-002"),
			Self::Custom { id, .. } => id.clone(),
			Self::Unknown(id) => Cow::Owned(id.to_owned()),
		}
	}

	/// Returns the human-readable display name for this model
	pub fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Gpt4o => Cow::Borrowed("GPT-4o"),
			Self::Gpt4oMini => Cow::Borrowed("GPT-4o Mini"),
			Self::TextEmbedding3Small => Cow::Borrowed("Text Embedding 3 Small"),
			Self::TextEmbedding3Large => Cow::Borrowed("Text Embedding 3 Large"),
			Self::TextEmbeddingAda002 => Cow::Borrowed("Text Embedding Ada 002"),
			Self::Custom { name, .. } => name.clone(),
			Self::Unknown(id) => Cow::Owned(format!("Unknown({id})")),
		}
	}

	/// Determines if this model supports text embedding operations
	pub const fn embedding(&self) -> bool {
		match self {
			Self::Gpt4o | Self::Gpt4oMini => false,
			Self::TextEmbedding3Small | Self::TextEmbedding3Large | Self::TextEmbeddingAda002 =>
				true,
			Self::Custom { embedding, .. } => *embedding,
			Self::Unknown(_) => false,
		}
	}

	/// Determines if this model supports reasoning capabilities
	pub const fn reasoning(&self) -> bool {
		match self {
			Self::Gpt4o
			| Self::Gpt4oMini
			| Self::TextEmbedding3Small
			| Self::TextEmbedding3Large
			| Self::TextEmbeddingAda002 => false,
			Self::Custom { reasoning, .. } => *reasoning,
			Self::Unknown(_) => false,
		}
	}

	/// Determines if this model supports function calling features
	pub const fn function_calling(&self) -> bool {
		match self {
			Self::Gpt4o | Self::Gpt4oMini => true,
			Self::TextEmbedding3Small | Self::TextEmbedding3Large | Self::TextEmbeddingAda002 =>
				false,
			Self::Custom { function_calling, .. } => *function_calling,
			Self::Unknown(_) => false,
		}
	}
}
impl Display for Model {
	/// Formats the model using its display name
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.name())
	}
}
impl Serialize for Model {
	/// Serializes the model using its identifier string
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.id())
	}
}
impl<'de> Deserialize<'de> for Model {
	/// Deserializes a model from identifier string, removing date suffixes
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		// Remove date suffix in format "-yyyy-mm-dd".
		let trimmed = RE_DATE_SUFFIX.replace(&s, "");

		Ok(Self::from_id(&trimmed))
	}
}
