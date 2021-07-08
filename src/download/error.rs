use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
	#[error(transparent)]
	NetworkConnection(#[from] ureq::Error),

	#[error(transparent)]
	Filesystem(#[from] crate::filesystem::FilesystemError),
}
