<div align="center">

# OpenAgent

### OpenAI API Agent Kit

A comprehensive Rust library providing type-safe, async interfaces to OpenAI APIs and advanced agent development capabilities. Build powerful AI applications with support for chat completions, embeddings, file operations, batch processing, and Model Context Protocol (MCP) integration.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Docs](https://img.shields.io/docsrs/openagent)](https://docs.rs/openagent)
[![Checks](https://github.com/hack-ink/openagent/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/hack-ink/openagent/actions/workflows/rust.yml)
[![Release](https://github.com/hack-ink/openagent/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/openagent/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/openagent)](https://github.com/hack-ink/openagent/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/openagent?color=red&style=plastic)](https://github.com/hack-ink/openagent)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/openagent)](https://github.com/hack-ink/openagent)
</div>

## Feature Highlights

🚀 **Complete OpenAI API Coverage**

- ✅ **Chat Completions API** - Full support for GPT models with streaming
- ✅ **Embeddings API** - Text embedding generation with multiple models
- ✅ **Files API** - Upload, retrieve, and manage files
- ✅ **Batch API** - Process requests in batches for cost optimization
- ✅ **Response API** - Advanced response handling with streaming support
- ✅ **Model Context Protocol (MCP)** - Integration with MCP servers

🔧 **Developer Experience**

- **Type Safety** - Fully typed Rust API with comprehensive error handling
- **Async/Await** - Built on tokio for high-performance async operations
- **Streaming Support** - Real-time streaming for chat and response APIs
- **Flexible Authentication** - Support for multiple OpenAI-compatible APIs
- **Event Handling** - Customizable event handlers for streaming data

🎯 **Agent Development**

- **Tool Integration** - Function calling and tool execution
- **Memory Management** - Built-in context and conversation management
- **Error Recovery** - Robust error handling and retry mechanisms
- **Multi-Modal Support** - Text, images, audio, and file inputs

## Usage

Set up your environment:

```bash
export OPENAI_API_KEY="your-api-key-here"
```

The library provides comprehensive examples for all supported APIs. Check out the [`examples/`](examples/) directory for complete, runnable code:

- **[`chat.rs`](examples/chat.rs)** - Chat completions with streaming support
- **[`embedding.rs`](examples/embedding.rs)** - Text embeddings generation
- **[`file.rs`](examples/file.rs)** - File upload and management
- **[`batch.rs`](examples/batch.rs)** - Batch processing for cost optimization
- **[`response.rs`](examples/response.rs)** - Advanced response API with real-time streaming
- **[`mcp.rs`](examples/mcp.rs)** - Model Context Protocol integration

Run any example with:

```bash
cargo run --example chat
cargo run --example response
cargo run --example embedding
# ... etc
```

## Development

### Architecture

The library is organized into several key modules:

- **`api/`** - Core API implementations for different OpenAI endpoints
  - `chat.rs` - Chat completions API with streaming support
  - `embedding.rs` - Text embeddings generation
  - `file.rs` - File upload and management
  - `batch.rs` - Batch processing operations
  - `response.rs` - Advanced response API with real-time streaming
  - `type.rs` - Common types and utilities

- **`http.rs`** - HTTP client abstraction with retry logic and streaming
- **`mcp.rs`** - Model Context Protocol integration
- **`tool.rs`** - Function calling and tool execution framework
- **`agent.rs`** - High-level agent abstraction (work in progress)
- **`error.rs`** - Comprehensive error handling

### Key Design Principles

1. **Type Safety** - Leverages Rust's type system for compile-time correctness
2. **Async-First** - Built on tokio for high-performance async operations
3. **Streaming Support** - Real-time data processing with Server-Sent Events
4. **Modularity** - Clean separation of concerns with trait-based design
5. **Compatibility** - Works with OpenAI and OpenAI-compatible APIs

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with tests
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## Support Me

If you find this project helpful and would like to support its development, you can buy me a coffee!

Your support is greatly appreciated and motivates me to keep improving this project.

- **Fiat**
  - [Ko-fi](https://ko-fi.com/hack_ink)
  - [爱发电](https://afdian.com/a/hack_ink)
- **Crypto**
  - **Bitcoin**
    - `bc1pedlrf67ss52md29qqkzr2avma6ghyrt4jx9ecp9457qsl75x247sqcp43c`
  - **Ethereum**
    - `0x3e25247CfF03F99a7D83b28F207112234feE73a6`
  - **Polkadot**
    - `156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y`

Thank you for your support!

## Appreciation

We would like to extend our heartfelt gratitude to the following projects and contributors:

- The Rust community for their continuous support and development of the Rust ecosystem.

## Additional Acknowledgements

Special thanks to the following projects and technologies that make OpenAgent possible:

- **[OpenAI](https://openai.com/)** - For providing the foundational APIs and documentation
- **[Model Context Protocol](https://modelcontextprotocol.io/)** - For the innovative protocol enabling tool integration

This project follows the principles of open-source development and aims to contribute back to the Rust ecosystem by providing a reliable, type-safe, and performant library for AI application development.

<div align="right">

### License

<sup>Licensed under [GPL-3.0](LICENSE).</sup>
</div>
