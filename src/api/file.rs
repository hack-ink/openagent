//! OpenAI Files API
//!
//! <https://platform.openai.com/docs/api-reference/files>

// self
use crate::_prelude::*;

/// OpenAI files API.
pub trait ApiFile
where
	Self: ApiBase,
{
	/// Upload a file.
	fn upload_file(
		&self,
		name: &str,
		content: Vec<u8>,
		purpose: Purpose,
	) -> impl Send + Future<Output = Result<FileObject>> {
		async move {
			let resp = self
				.post_multipart(
					"/files",
					Multipart {
						binary: vec![(
							Cow::Borrowed("file"),
							Cow::Owned(content),
							Some(name.into()),
						)],
						text: vec![(Cow::Borrowed("purpose"), Cow::Borrowed(purpose.as_str()))],
					},
				)
				.await?;

			tracing::debug!("{resp}");

			Ok(serde_json::from_str::<ApiResult<FileObject>>(&resp)?.as_result()?)
		}
	}

	/// Retrieve a file content by its ID.
	fn retrieve_file_content(&self, file_id: &str) -> impl Send + Future<Output = Result<String>> {
		async move {
			let resp = self.get(&format!("/files/{file_id}")).await?;

			tracing::debug!("{resp}");

			Ok(resp)
		}
	}
}
impl<T> ApiFile for T where T: ApiBase {}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
pub struct FileObject {
	pub bytes: u32,
	pub created_at: u64,
	pub expires_at: Option<u64>,
	// Can be ignored.
	// pub file: Option<()>,
	pub filename: String,
	pub id: String,
	pub object: String,
	pub purpose: String,
	pub status: StatusFallback,
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum StatusFallback {
	Completed,
	Fallback(String),
}
impl StatusFallback {
	#[allow(missing_docs)]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Completed => "completed",
			Self::Fallback(s) => s,
		}
	}

	#[allow(missing_docs)]
	pub fn completed(&self) -> bool {
		matches!(self, Self::Completed)
	}
}
impl<'de> Deserialize<'de> for StatusFallback {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;

		match s.as_str() {
			"completed" => Ok(Self::Completed),
			_ => Ok(Self::Fallback(s)),
		}
	}
}
