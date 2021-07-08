use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilesystemError {
	#[error("{path}: {source}")]
	Filesystem {
		source: std::io::Error,
		path: String,
	},
}

impl FilesystemError {
	pub fn from_io_error<S: Into<String>>(source: std::io::Error, path: S) -> Self {
		Self::Filesystem {
			source,
			path: path.into(),
		}
	}

	pub fn handling_io_error_in<S, F, R>(path: S, function: F) -> Result<R, Self>
	where
		S: Into<String>,
		F: FnOnce() -> Result<R, std::io::Error>,
	{
		function().map_err(|err| Self::from_io_error(err, path))
	}
}
