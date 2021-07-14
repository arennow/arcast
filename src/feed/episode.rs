use super::error::*;
use super::Show;
use chrono::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Builder)]
#[builder(setter(into))]
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
			self.show.title(),
			Self::formatted_string_for_date(&self.pub_date),
			self.process_raw_title()
		)
	}

	fn process_raw_title(&self) -> String {
		let default_patterns = [
			format!(r#"{}[:\s]+"#, self.show.title()),
			r#"^\s+|\s+$"#.into(),
		];

		let strip_patterns = default_patterns
			.iter()
			.chain(self.show.title_strip_patterns().iter());

		let mut processed_title: String =
			strip_patterns.fold(self.title.clone(), |title, raw_patt| {
				let reg = regex::Regex::new(raw_patt).expect("Bad regex");
				reg.replace_all(&title, "").into_owned()
			});

		unsafe {
			// This unsafe is safe because these two chars are the same length
			for byte in processed_title.as_bytes_mut().iter_mut() {
				if *byte == b'/' {
					*byte = b'-';
				}
			}
		}

		processed_title
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

#[cfg(test)]
mod tests {
	use super::super::ShowBuilder;
	use super::*;

	fn new_show(strip_patterns: Vec<&str>) -> Show {
		ShowBuilder::default()
			.title("FAKESHOW")
			.url("http://example.com/feed.rss")
			.title_strip_patterns(
				strip_patterns
					.into_iter()
					.map(|s| s.into())
					.collect::<Vec<String>>(),
			)
			.build()
			.unwrap()
	}

	fn new_episode(show: Show, title: &str) -> Episode {
		EpisodeBuilder::default()
			.show(Rc::new(show))
			.title(title)
			.pub_date(Local::now())
			.enclosure_url("http://example.com/ep.mp3")
			.cached_filename(RefCell::default())
			.build()
			.unwrap()
	}

	#[test]
	fn test_basic_stripping() {
		let show = new_show(vec![]);
		let ep = new_episode(show, "FAKESHOW 666: We fought the devil - LIVE EPISODE ");
		let raw_title = ep.process_raw_title();

		assert_eq!(raw_title, "666: We fought the devil - LIVE EPISODE")
	}

	#[test]
	fn test_regex_stripping() {
		let show = new_show(vec![r#"\s+-\s+LIVE EPISODE$"#]);
		let ep = new_episode(show, "FAKESHOW 666: We fought the devil - LIVE EPISODE");
		let raw_title = ep.process_raw_title();

		assert_eq!(raw_title, "666: We fought the devil")
	}
}
