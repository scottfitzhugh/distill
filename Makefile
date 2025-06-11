# Makefile for Distill - AI-Powered Git Commit Message Generator

# Variables
BINARY_NAME = distill
CARGO = cargo
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug

# Installation directories
PREFIX ?= /usr/local
LOCAL_PREFIX = $(HOME)/.local
INSTALL_DIR = $(PREFIX)/bin
LOCAL_INSTALL_DIR = $(LOCAL_PREFIX)/bin

# Default target
.PHONY: all
all: build

# Build targets
.PHONY: build
build: ## Build the project in debug mode
	$(CARGO) build

.PHONY: release
release: ## Build the project in release mode
	$(CARGO) build --release

.PHONY: debug
debug: build ## Alias for build (debug mode)

# Installation targets
.PHONY: install
install: release ## Install binary system-wide (requires sudo)
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@mkdir -p $(INSTALL_DIR)
	@cp $(RELEASE_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/
	@chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) installed successfully to $(INSTALL_DIR)"
	@echo "You can now run: $(BINARY_NAME)"

.PHONY: install-user
install-user: release ## Install binary for current user only
	@echo "Installing $(BINARY_NAME) to $(LOCAL_INSTALL_DIR)..."
	@mkdir -p $(LOCAL_INSTALL_DIR)
	@cp $(RELEASE_DIR)/$(BINARY_NAME) $(LOCAL_INSTALL_DIR)/
	@chmod +x $(LOCAL_INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) installed successfully to $(LOCAL_INSTALL_DIR)"
	@echo "Make sure $(LOCAL_INSTALL_DIR) is in your PATH"
	@echo "You can now run: $(BINARY_NAME)"

.PHONY: uninstall
uninstall: ## Uninstall system-wide binary
	@echo "Removing $(BINARY_NAME) from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) uninstalled successfully"

.PHONY: uninstall-user
uninstall-user: ## Uninstall user binary
	@echo "Removing $(BINARY_NAME) from $(LOCAL_INSTALL_DIR)..."
	@rm -f $(LOCAL_INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) uninstalled successfully"

# Development targets
.PHONY: run
run: ## Run the project with cargo run
	$(CARGO) run

.PHONY: run-release
run-release: ## Run the release version
	$(CARGO) run --release

.PHONY: test
test: ## Run tests
	$(CARGO) test

.PHONY: check
check: ## Check the project for errors without building
	$(CARGO) check

.PHONY: clippy
clippy: ## Run clippy for linting
	$(CARGO) clippy -- -D warnings

.PHONY: fmt
fmt: ## Format the code
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Check if code is formatted
	$(CARGO) fmt -- --check

# Utility targets
.PHONY: clean
clean: ## Clean build artifacts
	$(CARGO) clean
	@echo "✅ Build artifacts cleaned"

.PHONY: clean-all
clean-all: clean ## Clean all artifacts including Cargo registry cache
	$(CARGO) clean --release
	@echo "✅ All artifacts cleaned"

.PHONY: deps
deps: ## Update dependencies
	$(CARGO) update

.PHONY: audit
audit: ## Audit dependencies for security vulnerabilities
	$(CARGO) audit

.PHONY: tree
tree: ## Show dependency tree
	$(CARGO) tree

# Documentation targets
.PHONY: doc
doc: ## Build documentation
	$(CARGO) doc --no-deps

.PHONY: doc-open
doc-open: ## Build and open documentation
	$(CARGO) doc --no-deps --open

# Size and analysis targets
.PHONY: size
size: release ## Show binary size
	@echo "Binary size information:"
	@ls -lah $(RELEASE_DIR)/$(BINARY_NAME)
	@file $(RELEASE_DIR)/$(BINARY_NAME)

.PHONY: bloat
bloat: release ## Analyze binary bloat (requires cargo-bloat)
	@command -v cargo-bloat >/dev/null 2>&1 || { echo "Installing cargo-bloat..."; $(CARGO) install cargo-bloat; }
	$(CARGO) bloat --release

# Testing targets with different flags
.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	$(CARGO) test -- --nocapture

.PHONY: test-ignored
test-ignored: ## Run ignored tests
	$(CARGO) test -- --ignored

# Demo and help targets
.PHONY: demo
demo: release ## Run demo with dry-run flag
	@echo "Running demo (dry-run mode):"
	@echo "Note: Make sure OPENROUTER_API_KEY is set"
	./$(RELEASE_DIR)/$(BINARY_NAME) --dry-run --verbose

.PHONY: help
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Check for required tools
.PHONY: check-tools
check-tools: ## Check if required tools are available
	@echo "Checking required tools..."
	@command -v $(CARGO) >/dev/null 2>&1 || { echo "❌ Cargo not found. Please install Rust."; exit 1; }
	@command -v git >/dev/null 2>&1 || { echo "❌ Git not found. Please install Git."; exit 1; }
	@echo "✅ All required tools are available"

# Development setup
.PHONY: setup
setup: check-tools ## Setup development environment
	@echo "Setting up development environment..."
	$(CARGO) build
	@echo "✅ Development environment ready"
	@echo ""
	@echo "Next steps:"
	@echo "1. Set your OPENROUTER_API_KEY environment variable"
	@echo "2. Run 'make install-user' to install the binary"
	@echo "3. Run 'make demo' to test the application"

# CI targets
.PHONY: ci
ci: fmt-check clippy test ## Run all CI checks

.PHONY: pre-commit
pre-commit: fmt clippy test ## Run pre-commit checks

# Make help the default when no target is specified
.DEFAULT_GOAL := help 