use super::error::*;
use chrono::prelude::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Show {
	title: String,
	url: String,

	#[serde(default)]
	title_strip_patterns: Vec<String>,
}

impl Show {
	pub fn url(&self) -> &str {
		&self.url
	}
}

pub struct Episode {
	show: Rc<Show>,
	title: String,
	pub_date: DateTime<FixedOffset>,
	enclosure_url: String,
	cached_filename: RefCell<Option<Rc<String>>>,
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
			cached_filename: Default::default(),
		})
	}

	pub fn filename(&self) -> Rc<String> {
		if let Some(existing) = &*self.cached_filename.borrow() {
			return Rc::clone(existing);
		}

		let new = Rc::new(self.generate_filename());
		self.cached_filename.replace(Some(Rc::clone(&new)));

		new
	}

	fn generate_filename(&self) -> String {
		format!(
			"{} - {} - {}.mp3",
			self.show.title,
			Self::formatted_string_for_date(&self.pub_date),
			self.process_raw_title()
		)
	}

	fn process_raw_title(&self) -> String {
		let processed_title = self.show.title_strip_patterns.iter().fold(
			Cow::Borrowed(&self.title[..]),
			|title, raw_patt| {
				let reg = regex::Regex::new(raw_patt).expect("Bad regex");
				Cow::Owned(reg.replace_all(&title, "").into_owned())
			},
		);

		processed_title.to_string()
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
