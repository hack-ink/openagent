// std
use std::sync::LazyLock;
// crates.io
use regex::Regex;
// self
use crate::_prelude::*;

static RE_DATE_SUFFIX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"-\d{4}-\d{2}-\d{2}$").unwrap());

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Model {
	#[default]
	Gpt4o,
	Gpt4oMini,
	TextEmbedding3Small,
	TextEmbedding3Large,
	TextEmbeddingAda002,
	Custom {
		id: Cow<'static, str>,
		name: Cow<'static, str>,
		embedding: bool,
		reasoning: bool,
		function_calling: bool,
	},
	Unknown(String),
}
impl Model {
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

	pub const fn embedding(&self) -> bool {
		match self {
			Self::Gpt4o | Self::Gpt4oMini => false,
			Self::TextEmbedding3Small | Self::TextEmbedding3Large | Self::TextEmbeddingAda002 =>
				true,
			Self::Custom { embedding, .. } => *embedding,
			Self::Unknown(_) => false,
		}
	}

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
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.name())
	}
}
impl Serialize for Model {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.id())
	}
}
impl<'de> Deserialize<'de> for Model {
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
