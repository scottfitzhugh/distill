use anyhow::{Context, Result};
use clap::Parser;
use log::info;

mod config;
mod git;
mod openrouter;

use config::Config;
use git::GitManager;
use openrouter::OpenRouterClient;

/// AI-powered git commit message generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Don't automatically stage changes if no changes are currently staged
	#[arg(long)]
	no_auto_stage: bool,

	/// Dry run - generate commit message but don't commit
	#[arg(long)]
	dry_run: bool,

	/// Verbose output
	#[arg(short, long)]
	verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();

	// Initialize logger
	if args.verbose {
		env_logger::Builder::from_default_env()
			.filter_level(log::LevelFilter::Debug)
			.init();
	} else {
		env_logger::init();
	}

	// Load configuration
	let config = Config::load()?;
	info!("Configuration loaded successfully");

	// Initialize git manager
	let mut git_manager = GitManager::new(".")?;
	info!("Git repository initialized");

	// Check if there are staged changes
	let has_staged_changes = git_manager.has_staged_changes()?;
	info!("Staged changes detected: {}", has_staged_changes);

	if !has_staged_changes {
		if args.no_auto_stage {
			anyhow::bail!("No staged changes found and --no-auto-stage flag is set. Please stage some changes first.");
		}
		
		info!("No staged changes found, staging all changes...");
		git_manager.stage_all_changes()?;
		
		// Double-check that we have changes after staging
		if !git_manager.has_staged_changes()? {
			anyhow::bail!("No changes to commit after staging all files.");
		}
	}

	// Get the diff of staged changes
	let diff = git_manager.get_staged_diff()?;
	info!("Retrieved staged diff ({} characters)", diff.len());

	if diff.trim().is_empty() {
		anyhow::bail!("No staged changes found to generate commit message for.");
	}

	// Generate commit message using OpenRouter
	let openrouter_client = OpenRouterClient::new(&config.openrouter_api_key);
	info!("Generating commit message...");
	
	let commit_message = openrouter_client.generate_commit_message(&diff).await
		.context("Failed to generate commit message from OpenRouter API")?;

	println!("Generated commit message:");
	println!("------------------------");
	println!("{}", commit_message);
	println!("------------------------");

	if args.dry_run {
		println!("Dry run mode - commit message generated but not committed.");
		return Ok(());
	}

	// Commit with the generated message
	info!("Committing changes...");
	git_manager.commit(&commit_message)?;
	
	println!("✅ Successfully committed changes with AI-generated message!");
	
	Ok(())
} 