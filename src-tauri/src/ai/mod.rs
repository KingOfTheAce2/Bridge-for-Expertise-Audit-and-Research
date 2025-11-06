/// AI inference module for local LLM processing
///
/// This module provides the infrastructure for running large language models
/// locally using the Candle framework. It handles model loading, text generation,
/// streaming responses, and context management.

pub mod types;
pub mod inference;

pub use types::*;
pub use inference::InferenceEngine;
