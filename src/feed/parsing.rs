use super::{error::ParsingError, Episode, Show};
use rss::Channel;
use std::io::BufReader;
use std::io::Read;
use std::rc::Rc;

pub fn episodes_from_reader(reader: impl Read) -> Result<Vec<Episode>, ParsingError> {
	let channel = Channel::read_from(BufReader::new(reader))?;
	let show = Rc::new(Show::new(channel.title()));

	Ok(channel
		.into_items()
		.into_iter()
		.map(|rss_item| Episode::new(Rc::clone(&show), rss_item))
		.flatten()
		.collect())
}
