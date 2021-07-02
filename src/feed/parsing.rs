use rss::Channel;

use std::io::BufReader;
use std::io::Read;

use super::error::*;

pub fn items_from_reader(reader: impl Read) -> Result<Vec<rss::Item>, Error> {
	Ok(Channel::read_from(BufReader::new(reader))?
		.into_items()
		.into_iter()
		.filter(|i| i.enclosure != None)
		.collect())
}
