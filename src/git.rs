use anyhow::{Context, Result};
use git2::{Repository, Signature, Status, StatusOptions};
use log::{debug, warn};
use std::path::Path;

/// Git operations manager
pub struct GitManager {
	repo: Repository,
}

impl GitManager {
	/// Create a new GitManager for the repository at the given path
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let repo = Repository::open(path)
			.context("Failed to open git repository. Make sure you're in a git repository.")?;
		
		Ok(GitManager { repo })
	}

	/// Check if there are any staged changes
	pub fn has_staged_changes(&self) -> Result<bool> {
		let mut opts = StatusOptions::new();
		opts.include_ignored(false);
		opts.include_untracked(false);
		
		let statuses = self.repo.statuses(Some(&mut opts))
			.context("Failed to get git status")?;

		for entry in statuses.iter() {
			let status = entry.status();
			if status.intersects(
				Status::INDEX_NEW 
				| Status::INDEX_MODIFIED 
				| Status::INDEX_DELETED 
				| Status::INDEX_RENAMED 
				| Status::INDEX_TYPECHANGE
			) {
				return Ok(true);
			}
		}

		Ok(false)
	}

	/// Stage all changes in the repository
	pub fn stage_all_changes(&mut self) -> Result<()> {
		let mut index = self.repo.index()
			.context("Failed to get repository index")?;

		// Add all modified files
		index.add_all(&["*"], git2::IndexAddOption::DEFAULT, None)
			.context("Failed to stage all changes")?;

		// Write the index
		index.write()
			.context("Failed to write index after staging")?;

		debug!("Successfully staged all changes");
		Ok(())
	}

	/// Get the diff of currently staged changes
	pub fn get_staged_diff(&self) -> Result<String> {
		let head = self.repo.head()
			.context("Failed to get HEAD reference")?;
		
		let head_tree = head.peel_to_tree()
			.context("Failed to get HEAD tree")?;

		let mut index = self.repo.index()
			.context("Failed to get repository index")?;
		
		let index_tree = self.repo.find_tree(index.write_tree()?)
			.context("Failed to get index tree")?;

		let diff = self.repo.diff_tree_to_tree(
			Some(&head_tree),
			Some(&index_tree),
			None,
		).context("Failed to create diff between HEAD and index")?;

		let mut diff_output = String::new();
		
		diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
			match line.origin() {
				'+' | '-' | ' ' => {
					diff_output.push(line.origin());
					if let Ok(content) = std::str::from_utf8(line.content()) {
						diff_output.push_str(content);
					}
				}
				'F' => {
					// File header
					if let Ok(content) = std::str::from_utf8(line.content()) {
						diff_output.push_str("--- ");
						diff_output.push_str(content);
					}
				}
				'H' => {
					// Hunk header
					if let Ok(content) = std::str::from_utf8(line.content()) {
						diff_output.push_str("@@ ");
						diff_output.push_str(content);
					}
				}
				_ => {}
			}
			true
		}).context("Failed to generate diff output")?;

		debug!("Generated diff with {} characters", diff_output.len());
		Ok(diff_output)
	}

	/// Commit the staged changes with the given message
	pub fn commit(&self, message: &str) -> Result<()> {
		let signature = self.get_signature()
			.context("Failed to create git signature")?;

		let mut index = self.repo.index()
			.context("Failed to get repository index")?;

		let tree_id = index.write_tree()
			.context("Failed to write tree from index")?;
		
		let tree = self.repo.find_tree(tree_id)
			.context("Failed to find tree object")?;

		let head = self.repo.head()
			.context("Failed to get HEAD reference")?;
		
		let parent_commit = head.peel_to_commit()
			.context("Failed to get parent commit")?;

		self.repo.commit(
			Some("HEAD"),
			&signature,
			&signature,
			message,
			&tree,
			&[&parent_commit],
		).context("Failed to create commit")?;

		debug!("Successfully created commit with message: {}", message);
		Ok(())
	}

	/// Get the git signature for commits
	fn get_signature(&self) -> Result<Signature> {
		// Try to get signature from git config
		if let Ok(config) = self.repo.config() {
			if let (Ok(name), Ok(email)) = (
				config.get_string("user.name"),
				config.get_string("user.email")
			) {
				return Signature::now(&name, &email)
					.context("Failed to create signature from git config");
			}
		}

		warn!("Git user.name and user.email not configured, using default signature");
		
		// Fallback to default signature
		Signature::now("Distill", "distill@example.com")
			.context("Failed to create default signature")
	}
} 