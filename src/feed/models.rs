use super::error::*;
use chrono::prelude::*;
use std::rc::Rc;

pub struct Show {
	title: String,
}

impl Show {
	pub fn new<S: Into<String>>(title: S) -> Self {
		let title = title.into();

		Show { title }
	}
}

pub struct Episode {
	show: Rc<Show>,
	title: String,
	pub_date: DateTime<FixedOffset>,
	enclosure_url: String,
}

impl Episode {
	pub fn new(show: Rc<Show>, rss_item: rss::Item) -> Result<Self, ParsingError> {
		let title = rss_item
			.title()
			.ok_or(ParsingError::EpisodeTitleMissing)?
			.into();

		let string_pub_date = rss_item
			.pub_date()
			.ok_or(ParsingError::EpisodePubDateMissing)?;
		let pub_date = DateTime::parse_from_rfc2822(string_pub_date)?;

		let enclosure_url = rss_item
			.enclosure()
			.ok_or(ParsingError::EpisodeEnclosureURLMissing)?
			.url()
			.into();

		Ok(Episode {
			show,
			title,
			pub_date,
			enclosure_url,
		})
	}

	pub fn filename(&self) -> String {
		format!(
			"{} - {} - {}.mp3",
			self.show.title,
			Self::formatted_string_for_date(&self.pub_date),
			self.title
		)
	}

	fn formatted_string_for_date(date: &DateTime<FixedOffset>) -> impl std::fmt::Display {
		date.format("%F")
		// Year-month-day format (ISO 8601). Same as %Y-%m-%d. (https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
	}
}

impl Episode {
	pub fn enclosure_url(&self) -> &str {
		&self.enclosure_url
	}
}
