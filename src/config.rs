use anyhow::{Context, Result};
use std::env;

/// Configuration for the application
#[derive(Debug)]
pub struct Config {
	pub openrouter_api_key: String,
}

impl Config {
	/// Load configuration from environment variables
	pub fn load() -> Result<Self> {
		let openrouter_api_key = env::var("OPENROUTER_API_KEY")
			.context("OPENROUTER_API_KEY environment variable is not set. Please set it to your OpenRouter API key.")?;

		if openrouter_api_key.trim().is_empty() {
			anyhow::bail!("OPENROUTER_API_KEY environment variable is empty. Please provide a valid API key.");
		}

		Ok(Config {
			openrouter_api_key,
		})
	}
} 