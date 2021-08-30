use super::{error::ParsingError, Episode, Show};
use rss::Channel;
use std::io::BufReader;
use std::io::Read;

pub fn episodes_from_reader(reader: impl Read, show: &Show) -> Result<Vec<Episode>, ParsingError> {
	let channel = Channel::read_from(BufReader::new(reader))?;

	Ok(channel
		.into_items()
		.into_iter()
		.flat_map(|rss_item| Episode::new(show, &rss_item))
		.collect())
}
