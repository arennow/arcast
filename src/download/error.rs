use crate::filesystem::FilesystemError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
	#[error(transparent)]
	NetworkConnection(#[from] ureq::Error),

	#[error(transparent)]
	Filesystem(#[from] crate::filesystem::FilesystemError),
}

impl From<ureq::Error> for Box<DownloadError> {
	fn from(src: ureq::Error) -> Self {
		Box::new(From::from(src))
	}
}

impl From<FilesystemError> for Box<DownloadError> {
	fn from(src: FilesystemError) -> Self {
		Box::new(From::from(src))
	}
}
