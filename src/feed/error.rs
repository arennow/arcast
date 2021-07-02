use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Parsing(#[from] rss::Error),
}
