# Distill - AI-Powered Git Commit Message Generator

A Rust command-line application that automatically generates intelligent commit messages using OpenRouter API based on your staged git changes.

## Features

- 🤖 **AI-Generated Commit Messages**: Uses OpenRouter API to generate contextual commit messages
- 🔍 **Smart Change Detection**: Analyzes staged changes or auto-stages all changes
- 🚀 **One-Command Workflow**: Generate and commit in a single command
- 🛡️ **Safety First**: Dry-run mode to preview commit messages before committing
- 📝 **Conventional Commits**: Follows conventional commit format standards
- ⚙️ **Configurable**: Flexible command-line options

## Installation

### Prerequisites

- Rust (1.70 or later)
- Git repository
- OpenRouter API key

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd distill

# Build the application
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Configuration

### Environment Variables

Set your OpenRouter API key:

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

To make this permanent, add it to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
echo 'export OPENROUTER_API_KEY="your-api-key-here"' >> ~/.zshrc
source ~/.zshrc
```

### Git Configuration

Ensure your git user information is configured:

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

## Usage

### Basic Usage

```bash
# Generate commit message and commit automatically
distill

# Preview commit message without committing (dry run)
distill --dry-run

# Don't auto-stage changes - only work with already staged changes
distill --no-auto-stage

# Verbose output for debugging
distill --verbose
```

### Command-Line Options

- `--no-auto-stage`: Don't automatically stage changes if no changes are staged
- `--dry-run`: Generate commit message but don't commit
- `--verbose, -v`: Enable verbose logging output
- `--help, -h`: Show help information

### Workflow Examples

#### Standard Workflow
```bash
# Make your changes
echo "console.log('Hello, World!');" > hello.js

# Generate AI commit message and commit
distill
```

#### Selective Staging
```bash
# Stage specific files
git add src/main.rs

# Generate commit message for staged changes only
distill --no-auto-stage
```

#### Preview Mode
```bash
# See what commit message would be generated
distill --dry-run

# If you like it, run again without --dry-run
distill
```

## How It Works

1. **Change Detection**: Checks for staged changes in your git repository
2. **Auto-Staging**: If no changes are staged (and `--no-auto-stage` isn't set), stages all changes
3. **Diff Generation**: Creates a diff of the staged changes
4. **AI Processing**: Sends the diff to OpenRouter API for commit message generation
5. **Commit Creation**: Creates a git commit with the AI-generated message

## Configuration

The application uses the OpenRouter API with the `meta-llama/llama-3.1-8b-instruct:free` model by default. The AI is prompted to generate commit messages following conventional commit standards.

### Commit Message Format

Generated commit messages follow the conventional commit format:
- `feat: add new feature`
- `fix: resolve bug in authentication`
- `docs: update API documentation`
- `refactor: restructure user service`

## Error Handling

The application provides clear error messages for common issues:

- **Missing API Key**: Clear instructions to set `OPENROUTER_API_KEY`
- **Not in Git Repository**: Ensures you're running from within a git repository
- **No Changes**: Handles cases where there are no changes to commit
- **API Failures**: Graceful handling of network or API issues

## Troubleshooting

### Common Issues

1. **"OPENROUTER_API_KEY environment variable is not set"**
   - Solution: Set your API key as shown in the Configuration section

2. **"Failed to open git repository"**
   - Solution: Ensure you're running the command from within a git repository

3. **"No changes to commit"**
   - Solution: Make some changes to your files or check if you have uncommitted changes

4. **API request failures**
   - Solution: Check your internet connection and API key validity

### Debug Mode

Use the `--verbose` flag to see detailed logs:

```bash
distill --verbose
```

This will show:
- Git operations
- API requests and responses
- Internal processing steps

## Development

### Project Structure

```
src/
├── main.rs          # CLI interface and orchestration
├── config.rs        # Configuration and environment variables
├── git.rs           # Git operations (staging, diff, commit)
└── openrouter.rs    # OpenRouter API client
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logs
RUST_LOG=debug cargo run -- --verbose
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- OpenRouter for providing the AI API
- The Rust community for excellent crates
- Git for being an amazing version control system 