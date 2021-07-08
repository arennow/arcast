use super::FilesystemError;
use std::collections::hash_set::HashSet;
use std::fs::read_dir;
use std::path::Path;

pub fn list_files<P: AsRef<Path>>(path: P) -> Result<HashSet<String>, FilesystemError> {
	let path_str = path.as_ref().to_string_lossy();
	FilesystemError::handling_io_error_in(path_str, || {
		Ok(read_dir(&path)?
			.flatten()
			.filter(|file| match file.file_type() {
				Err(_) => false,
				Ok(ft) => ft.is_file(),
			})
			.map(|file| file.file_name().to_string_lossy().into_owned())
			.collect())
	})
}
