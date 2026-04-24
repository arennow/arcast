use crate::filesystem::{hash_file, list_files, FilesystemError};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Scans `destination` for files that are byte-for-byte duplicates of a file
/// whose name appears in `canonical_filenames`, but whose own name does *not*
/// appear in `canonical_filenames`.
///
/// The motivating case: an episode originally downloaded as "Old Name.mp3"
/// that was later renamed to "New Name.mp3" in the feed.  After the rename the
/// destination directory contains two identical files.  Calling this function
/// with `canonical_filenames` built from the current feed returns the path of
/// "Old Name.mp3" so the caller can remove it.
///
/// Safety rule: a group must contain *at least one* canonical filename before
/// any non-canonical files in that group are flagged.  If no canonical file
/// exists in the group (e.g. an episode that was removed from the feed
/// entirely), nothing is flagged, preventing accidental deletion.
pub fn find_files_to_prune(
	destination: &Path,
	canonical_filenames: &HashSet<String>,
) -> Result<Vec<PathBuf>, FilesystemError> {
	let existing: Vec<String> = list_files(destination)?.into_iter().collect();

	// Hash all files in parallel (CPU-bound), then collect results.
	let hashed: Result<Vec<([u8; 32], String)>, FilesystemError> = existing
		.par_iter()
		.map(|filename| {
			let path = destination.join(filename);
			hash_file(&path).map(|digest| (digest, filename.clone()))
		})
		.collect();

	// Group filenames by digest.
	let mut groups: HashMap<[u8; 32], Vec<String>> = HashMap::new();
	for (digest, filename) in hashed? {
		groups.entry(digest).or_default().push(filename);
	}

	let mut to_prune = Vec::new();
	for (_digest, filenames) in groups {
		// Only groups with more than one file can have duplicates.
		if filenames.len() <= 1 {
			continue;
		}
		// Only act when at least one file in the group has a canonical name.
		let has_canonical = filenames.iter().any(|f| canonical_filenames.contains(f));
		if !has_canonical {
			continue;
		}
		for filename in filenames {
			if !canonical_filenames.contains(&filename) {
				to_prune.push(destination.join(&filename));
			}
		}
	}

	Ok(to_prune)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;
	use std::fs;
	use tempfile::TempDir;

	fn write(dir: &TempDir, name: &str, contents: &[u8]) -> PathBuf {
		let path = dir.path().join(name);
		fs::write(&path, contents).unwrap();
		path
	}

	fn canonical(names: &[&str]) -> HashSet<String> {
		names.iter().map(|s| s.to_string()).collect()
	}

	/// One canonical file and one non-canonical with identical bytes → prune
	/// the non-canonical one.
	#[test]
	fn test_prune_renamed_duplicate() {
		let dir = TempDir::new().unwrap();
		write(&dir, "New Name.mp3", b"audio data");
		write(&dir, "Old Name.mp3", b"audio data");

		let to_prune = find_files_to_prune(dir.path(), &canonical(&["New Name.mp3"])).unwrap();

		assert_eq!(to_prune.len(), 1);
		assert_eq!(to_prune[0], dir.path().join("Old Name.mp3"));
	}

	/// Two identical files but neither is canonical → nothing should be pruned
	/// (safety guard: we won't delete files unless we're sure the episode is
	/// still present under its current feed name).
	#[test]
	fn test_no_canonical_in_group_is_safe() {
		let dir = TempDir::new().unwrap();
		write(&dir, "File A.mp3", b"audio data");
		write(&dir, "File B.mp3", b"audio data");

		// Neither name is canonical.
		let to_prune = find_files_to_prune(dir.path(), &canonical(&[])).unwrap();

		assert!(to_prune.is_empty());
	}

	/// Three identical files, one canonical → the two non-canonical files are
	/// both returned for pruning.
	#[test]
	fn test_multiple_non_canonical_returned() {
		let dir = TempDir::new().unwrap();
		write(&dir, "Current.mp3", b"audio data");
		write(&dir, "Old Name 1.mp3", b"audio data");
		write(&dir, "Old Name 2.mp3", b"audio data");

		let mut to_prune = find_files_to_prune(dir.path(), &canonical(&["Current.mp3"])).unwrap();
		to_prune.sort();

		assert_eq!(to_prune.len(), 2);
		assert!(to_prune.contains(&dir.path().join("Old Name 1.mp3")));
		assert!(to_prune.contains(&dir.path().join("Old Name 2.mp3")));
	}

	/// Files with different content are not duplicates and must never be
	/// flagged for pruning.
	#[test]
	fn test_unique_files_not_pruned() {
		let dir = TempDir::new().unwrap();
		write(&dir, "Episode 1.mp3", b"audio data episode one");
		write(&dir, "Episode 2.mp3", b"audio data episode two");

		let to_prune =
			find_files_to_prune(dir.path(), &canonical(&["Episode 1.mp3", "Episode 2.mp3"]))
				.unwrap();

		assert!(to_prune.is_empty());
	}

	/// A lone file with no duplicate is never flagged regardless of whether it
	/// is canonical.
	#[test]
	fn test_single_file_not_pruned() {
		let dir = TempDir::new().unwrap();
		write(&dir, "Episode 1.mp3", b"audio data");

		let to_prune = find_files_to_prune(dir.path(), &canonical(&["Episode 1.mp3"])).unwrap();

		assert!(to_prune.is_empty());
	}

	/// If every file in a duplicate group is canonical, none should be pruned.
	/// (Theoretically two distinct feed episodes could share identical bytes;
	/// we must not delete either.)
	#[test]
	fn test_both_canonical_not_pruned() {
		let dir = TempDir::new().unwrap();
		write(&dir, "Episode A.mp3", b"audio data");
		write(&dir, "Episode B.mp3", b"audio data");

		let to_prune =
			find_files_to_prune(dir.path(), &canonical(&["Episode A.mp3", "Episode B.mp3"]))
				.unwrap();

		assert!(to_prune.is_empty());
	}

	/// An empty destination directory produces an empty result without error.
	#[test]
	fn test_empty_directory() {
		let dir = TempDir::new().unwrap();
		let to_prune = find_files_to_prune(dir.path(), &canonical(&["Episode.mp3"])).unwrap();
		assert!(to_prune.is_empty());
	}
}
