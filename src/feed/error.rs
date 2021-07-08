use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
	#[error(transparent)]
	Document(#[from] rss::Error),

	#[error("episode missing title")]
	EpisodeTitleMissing,

	#[error("episode missing pubDate")]
	EpisodePubDateMissing,

	#[error(transparent)]
	EpisodePubDate(#[from] chrono::ParseError),

	#[error("episode missing URL")]
	EpisodeEnclosureURLMissing,
}
