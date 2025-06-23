//! HTTP client abstraction and implementations.

// std
use std::{
	env,
	fmt::Debug,
	io::{Error as IoError, Result as IoResult},
	mem,
	pin::Pin,
	task::{Context, Poll},
	time::Duration,
};
// crates.io
use futures::{Stream, TryStreamExt};
use reqwew::{
	Http,
	reqwest::{
		Body, Client as ReqwestClient, Method,
		multipart::{Form, Part},
	},
};
use tokio_util::{
	bytes::Bytes,
	codec::{FramedRead, LinesCodec},
	io::StreamReader,
};
// self
use crate::_prelude::*;

pub(crate) type EventStream<T> = _Stream<Result<T>>;

type _Stream<T> = Pin<Box<dyn Send + Stream<Item = T>>>;
type ByteStream = _Stream<IoResult<Bytes>>;

/// HTTP abstraction for making requests.
pub trait ApiBase
where
	Self: Send + Sync,
{
	/// Get the base URI for the API.
	fn base_uri(&self) -> &str;

	/// Make a non-streaming GET request.
	fn get(&self, endpoint: &str) -> impl Send + Future<Output = Result<String>>;

	fn post_multipart(
		&self,
		endpoint: &str,
		multipart: Multipart,
	) -> impl Send + Future<Output = Result<String>>;

	/// Make a non-streaming POST request with JSON body.
	fn post_json<S>(&self, endpoint: &str, body: S) -> impl Send + Future<Output = Result<String>>
	where
		S: Send + Serialize;

	/// Make a streaming POST request.
	fn sse<S, H>(
		&self,
		endpoint: &str,
		body: S,
		options: SseOptions<H>,
	) -> impl Send + Future<Output = Result<EventStream<H::Event>>>
	where
		S: Send + Serialize,
		H: 'static + EventHandler;

	/// Make a streaming POST request with resumption support.
	///
	/// The `last_event_id` parameter allows resuming from a specific event ID.
	fn sse_with_resume<S, H>(
		&self,
		endpoint: &str,
		body: S,
		options: SseOptions<H>,
		last_event_id: Option<&str>,
	) -> impl Send + Future<Output = Result<EventStream<H::Event>>>
	where
		S: Send + Serialize,
		H: 'static + EventHandler;
}

/// Trait for handling events in the SSE stream.
pub trait EventHandler
where
	Self: Send,
{
	type Event;

	/// Handle an event from the SSE stream.
	///
	/// This is called when an "event" is received.
	fn handle_event(&self, #[allow(unused)] event: &str) -> Result<()> {
		Ok(())
	}

	/// Handle data from the SSE stream.
	///
	/// This is called when the "data" field is received.
	fn handle_data(&self, data: String) -> Result<Self::Event>;

	/// Handle unexpected content in the SSE stream.
	///
	/// Ignored by default, but can be overridden to handle unexpected content.
	fn handle_unexpected(&self, #[allow(unused)] unexpected: String) -> Result<()> {
		Ok(())
	}
}
impl EventHandler for () {
	type Event = String;

	fn handle_data(&self, data: String) -> Result<Self::Event> {
		Ok(data)
	}
}

/// Options for the SSE stream.
#[derive(Debug)]
pub struct SseOptions<H> {
	/// Whether to drop the "event" and only process the "data".
	pub drop_event: bool,
	/// The event handler for processing events from the SSE stream.
	pub event_handler: H,
	/// Options for reconnecting to the SSE stream.
	pub reconnect: Reconnect,
}
impl<H> SseOptions<H> {
	/// Create a new [`SseOptions`] with the given event handler.
	pub fn new(event_handler: H) -> Self {
		Self { drop_event: false, event_handler, reconnect: Reconnect::default() }
	}

	/// Set the drop event option.
	pub fn drop_event(mut self, drop: bool) -> Self {
		self.drop_event = drop;

		self
	}

	/// Set the event handler for processing events from the SSE stream.
	pub fn event_handler(mut self, event_handler: H) -> Self {
		self.event_handler = event_handler;

		self
	}

	/// Set the options for reconnecting to the SSE stream.
	pub fn reconnect(mut self, reconnect: Reconnect) -> Self {
		self.reconnect = reconnect;

		self
	}
}

/// Options for reconnecting to the SSE stream.
#[derive(Debug)]
pub struct Reconnect {
	/// Whether to support reconnection.
	pub support: bool,
	/// Maximum number of retries for reconnection.
	pub max_retries: usize,
	/// Interval between retries for reconnection.
	pub retry_interval: Duration,
}
impl Default for Reconnect {
	fn default() -> Self {
		Self { support: false, max_retries: 3, retry_interval: Duration::from_millis(200) }
	}
}

#[derive(Clone, Debug)]
pub struct Api {
	http: ReqwestClient,
	auth: Auth,
}
impl Api {
	pub fn new(auth: Auth) -> Self {
		let http = ReqwestClient::builder()
			.user_agent("openagent")
			.build()
			.expect("build must succeed; qed");

		Self { http, auth }
	}
}
impl ApiBase for Api {
	fn base_uri(&self) -> &str {
		&self.auth.uri
	}

	async fn get(&self, endpoint: &str) -> Result<String> {
		let resp = self
			.http
			.request_with_retries(
				self.http
					.request(Method::GET, format!("{}{endpoint}", self.base_uri()))
					.bearer_auth(&self.auth.key)
					.build()?,
				3,
				200,
			)
			.await?;
		let text = resp.text().await?;

		Ok(text)
	}

	async fn post_multipart(&self, endpoint: &str, multipart: Multipart) -> Result<String> {
		let resp = <ReqwestClient as Http>::request(
			&self.http,
			self.http
				.request(Method::POST, format!("{}{endpoint}", self.base_uri()))
				.bearer_auth(&self.auth.key)
				.multipart(multipart.into())
				.build()?,
		)
		.await?;
		let text = resp.text().await?;

		Ok(text)
	}

	async fn post_json<S>(&self, endpoint: &str, body: S) -> Result<String>
	where
		S: Send + Serialize,
	{
		let resp = self
			.http
			.request_with_retries(
				self.http
					.request(Method::POST, format!("{}{endpoint}", self.base_uri()))
					.bearer_auth(&self.auth.key)
					.json(&body)
					.build()?,
				3,
				200,
			)
			.await?;
		let text = resp.text().await?;

		Ok(text)
	}

	async fn sse<S, H>(
		&self,
		endpoint: &str,
		body: S,
		options: SseOptions<H>,
	) -> Result<Pin<Box<dyn Send + Stream<Item = Result<H::Event>>>>>
	where
		S: Send + Serialize,
		H: 'static + EventHandler,
	{
		let req = self
			.http
			.request(Method::POST, format!("{}{endpoint}", self.base_uri()))
			.bearer_auth(&self.auth.key)
			.header("Accept", "text/event-stream")
			.header("Cache-Control", "no-cache")
			.json(&body);
		let stream = self
			.http
			.request_with_retries(req.build()?, 3, 200)
			.await?
			.bytes_stream()
			.map_err(IoError::other);
		let reader = StreamReader::new(Box::pin(stream) as _);
		let stream = FramedRead::new(reader, LinesCodec::new());

		Ok(Box::pin(Sse {
			stream,
			options,
			last_event: Default::default(),
			data: Default::default(),
			unexpected: Default::default(),
		}))
	}

	async fn sse_with_resume<S, H>(
		&self,
		endpoint: &str,
		body: S,
		options: SseOptions<H>,
		last_event_id: Option<&str>,
	) -> Result<Pin<Box<dyn Send + Stream<Item = Result<H::Event>>>>>
	where
		S: Send + Serialize,
		H: 'static + EventHandler,
	{
		let mut req = self
			.http
			.request(Method::POST, format!("{}{endpoint}", self.base_uri()))
			.bearer_auth(&self.auth.key)
			.header("Accept", "text/event-stream")
			.header("Cache-Control", "no-cache")
			.json(&body);
		// Add Last-Event-ID header for resumption.
		if let Some(event_id) = last_event_id {
			req = req.header("Last-Event-ID", event_id);
		}
		let stream = self
			.http
			.request_with_retries(req.build()?, 3, 200)
			.await?
			.bytes_stream()
			.map_err(IoError::other);
		let reader = StreamReader::new(Box::pin(stream) as _);
		let stream = FramedRead::new(reader, LinesCodec::new());

		Ok(Box::pin(Sse {
			stream,
			options,
			last_event: (None, last_event_id.map(Into::into)),
			data: Default::default(),
			unexpected: Default::default(),
		}))
	}
}

/// Authentication information for the API.
#[derive(Clone, Debug)]
pub struct Auth {
	/// The base URI for the API.
	pub uri: String,
	/// The API key for authentication.
	pub key: String,
}
impl Auth {
	/// Create a new [`Auth`] instance with the given URI and key.
	pub fn from_env() -> Self {
		Auth {
			uri: env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL must be set; qed"),
			key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set; qed"),
		}
	}
}

/// Multipart data for requests that require both binary and text parts.
#[derive(Clone, Debug, Default)]
pub struct Multipart {
	/// Binary parts of the multipart request.
	///
	/// Each tuple contains:
	/// - The name of the part (as a `Cow<'static, str>`).
	/// - The binary data (as a `Cow<'static, [u8]>`).
	/// - An optional filename (as an `Option<String>`).
	#[allow(clippy::type_complexity)]
	pub binary: Vec<(Cow<'static, str>, Cow<'static, [u8]>, Option<String>)>,
	/// Text parts of the multipart request.
	pub text: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}
impl From<Multipart> for Form {
	fn from(val: Multipart) -> Form {
		val.binary.into_iter().fold(
			val.text.into_iter().fold(Form::new(), |form, (k, v)| form.text(k, v)),
			|form, (k, v, filename)| {
				let len = v.len() as _;

				form.part(
					k,
					match v {
						Cow::Borrowed(v) => build_stream_part(v, len, filename),
						Cow::Owned(v) => build_stream_part(v, len, filename),
					},
				)
			},
		)
	}
}

/// Server-Sent Events (SSE) stream implementation.
#[pin_project::pin_project]
pub struct Sse<T> {
	/// The stream of lines read from the SSE response.
	#[pin]
	pub stream: FramedRead<StreamReader<ByteStream, Bytes>, LinesCodec>,
	/// Options for the SSE stream.
	pub options: SseOptions<T>,
	/// The last event: (event_type, event_id)
	///
	/// This stores the current event type and ID for processing and resumption.
	pub last_event: (Option<String>, Option<String>),
	/// Buffer for accumulating data chunks from multiple `data:` lines.
	///
	/// SSE events can span multiple `data:` lines. This field accumulates
	/// all data chunks until an empty line is encountered, at which point
	/// the complete accumulated data is passed to the event handler.
	///
	/// Memory consideration: For memory-sensitive applications where data
	/// can be very large, consider using [`String::with_capacity()`] or
	/// implementing a streaming data handler that processes data incrementally.
	pub data: String,
	/// Buffer for accumulating non-SSE formatted content (like raw JSON errors).
	///
	/// Some servers may return error responses as raw JSON without SSE formatting.
	/// This field accumulates such content to be parsed as a complete response.
	pub unexpected: String,
}
impl<T> Stream for Sse<T>
where
	T: EventHandler,
{
	type Item = Result<T::Event>;

	fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
		let mut this = self.project();

		loop {
			match Pin::new(&mut this.stream).poll_next(ctx) {
				Poll::Ready(Some(Ok(line))) => {
					let line = line.trim();

					// Handle SSE protocol.
					if line.is_empty() {
						// Empty line indicates end of an event.
						if !this.data.is_empty() {
							let data = mem::take(this.data);

							// Shrink capacity to free unused memory if the string was large.
							this.data.shrink_to_fit();

							let res = this.options.event_handler.handle_data(data);

							// Clear current event type.
							this.last_event.0 = None;

							return Poll::Ready(Some(res));
						}

						continue;
					}

					tracing::debug!("{line}");

					// Parse SSE line.
					if let Some(data_chunk) = line.strip_prefix("data: ") {
						if data_chunk == "[DONE]" {
							return Poll::Ready(None);
						}

						// Accumulate data.
						if !this.data.is_empty() {
							this.data.push('\n');
						}

						this.data.push_str(data_chunk);
					} else if let Some(event) = line.strip_prefix("event: ") {
						// Handle event.
						if !this.options.drop_event {
							this.last_event.0 = Some(event.into());

							if let Err(e) = this.options.event_handler.handle_event(event) {
								return Poll::Ready(Some(Err(e)));
							}
						}
					} else if let Some(event_id) = line.strip_prefix("id: ") {
						// Store event ID for reconnection.
						this.last_event.1 = Some(event_id.into());
					} else if let Some(retry_ms) = line.strip_prefix("retry: ") {
						// Handle retry instruction (optional implementation).
						if let Ok(_ms) = retry_ms.parse::<u64>() {
							// Update retry interval if needed (currently ignored).
						}
					} else if line.starts_with(':') {
						// Comment line, ignore.
						continue;
					} else {
						// Non-SSE formatted line - accumulate as unexpected content.
						if !this.unexpected.is_empty() {
							this.unexpected.push('\n');
						}

						this.unexpected.push_str(line);
					}
				},
				Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
				Poll::Ready(None) => {
					// Stream ended - check if we have accumulated unexpected content to process.
					if !this.unexpected.is_empty() {
						let unexpected = mem::take(this.unexpected);

						this.unexpected.shrink_to_fit();

						if let Err(e) = this.options.event_handler.handle_unexpected(unexpected) {
							return Poll::Ready(Some(Err(e)));
						}
					}

					return Poll::Ready(None);
				},
				Poll::Pending => return Poll::Pending,
			}
		}
	}
}

fn build_stream_part<T>(data: T, data_len: u64, filename: Option<String>) -> Part
where
	T: Into<Body>,
{
	let part = Part::stream_with_length(data, data_len);

	if let Some(filename) = filename { part.file_name(filename) } else { part }
}
