# Git Commit Message Generator - Project Plan

## Overview
A Rust command-line application that automatically generates commit messages using OpenRouter API based on currently staged git changes.

## Features
- Analyzes currently staged git changes
- Auto-stages all changes if no changes are staged (configurable via flag)
- Generates intelligent commit messages using OpenRouter API
- Automatically commits with the generated message
- Environment variable based API key configuration
- Error handling for missing API keys and git operations

## Architecture
- **Main Module**: Entry point and CLI argument parsing
- **Git Module**: Git operations (staging, diff, commit)
- **OpenRouter Module**: API communication for message generation
- **Config Module**: Environment variable and configuration management

## Dependencies
- `clap`: Command-line argument parsing
- `tokio`: Async runtime
- `reqwest`: HTTP client for API requests
- `serde`: JSON serialization/deserialization
- `git2`: Git operations
- `anyhow`: Error handling

## Usage
```bash
# Basic usage - auto-stage and commit
distill

# Don't auto-stage changes
distill --no-auto-stage

# Help
distill --help
```

## Environment Variables
- `OPENROUTER_API_KEY`: Required API key for OpenRouter service

## Database Schema
N/A - This application doesn't use a database.

## Milestones
- [x] Project planning
- [x] Basic project structure
- [x] Git operations implementation
- [x] OpenRouter API integration
- [x] CLI interface
- [x] Error handling
- [x] Testing
- [x] Documentation

## Implementation Summary

The Rust application has been successfully implemented with the following key components:

### Core Features Implemented
1. **Git Operations** (`src/git.rs`):
   - Detects staged changes
   - Auto-stages all changes when needed
   - Generates diffs of staged changes
   - Creates commits with generated messages
   - Handles git signatures from config

2. **OpenRouter API Integration** (`src/openrouter.rs`):
   - Uses `meta-llama/llama-3.1-8b-instruct:free` model
   - Sends structured prompts for commit message generation
   - Handles API errors gracefully
   - Truncates large diffs to avoid token limits

3. **Configuration Management** (`src/config.rs`):
   - Validates OPENROUTER_API_KEY environment variable
   - Provides clear error messages for missing configuration

4. **CLI Interface** (`src/main.rs`):
   - `--no-auto-stage`: Prevents automatic staging
   - `--dry-run`: Preview mode without committing
   - `--verbose`: Debug output
   - Comprehensive error handling and user feedback

### Build & Test Results
- ✅ Compiles successfully in release mode
- ✅ CLI help menu displays correctly
- ✅ Error handling works for missing API key
- ✅ All dependencies resolved correctly 