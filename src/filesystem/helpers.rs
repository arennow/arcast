use super::FilesystemError;
use sha2::{Digest, Sha256};
use std::collections::hash_set::HashSet;
use std::fs::read_dir;
use std::io::Read;
use std::path::Path;

pub fn list_files<P: AsRef<Path>>(path: P) -> Result<HashSet<String>, FilesystemError> {
	let path_str = path.as_ref().to_string_lossy();
	FilesystemError::handling_io_error_in(path_str, || {
		Ok(read_dir(&path)?
			.flatten()
			.filter(|file| file.file_type().map_or(false, |ft| ft.is_file()))
			.map(|file| file.file_name().to_string_lossy().into_owned())
			.collect())
	})
}

/// SHA-256 hash the contents of a file. The 32-byte digest is returned as a
/// fixed-size array so it can be used directly as a `HashMap` key.
pub fn hash_file(path: &Path) -> Result<[u8; 32], FilesystemError> {
	let path_str = path.to_string_lossy();
	FilesystemError::handling_io_error_in(path_str, || {
		let mut file = std::fs::File::open(path)?;
		let mut hasher = Sha256::new();
		let mut buf = [0u8; 8192];
		loop {
			let n = file.read(&mut buf)?;
			if n == 0 {
				break;
			}
			hasher.update(&buf[..n]);
		}
		Ok(hasher.finalize().into())
	})
}
