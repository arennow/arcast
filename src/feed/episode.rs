use super::{error::*, RegexContainer, Show};
use chrono::prelude::*;
use derive_getters::Getters;
use regex::Regex;
use std::borrow::Cow;
use std::rc::Rc;

lazy_static! {
	static ref EDGE_TRIM_REGEX: Regex = Regex::new(r#"^\s+|\s+$"#).unwrap();
}

#[derive(Builder, Getters, Debug)]
#[builder(setter(into), pattern = "owned")]
pub struct Episode {
	enclosure_url: String,
	filename: String,
}

impl Episode {
	pub fn new(show: &Show, rss_item: rss::Item) -> Result<Self, ParsingError> {
		let mut title = Cow::Borrowed(rss_item.title().ok_or(ParsingError::EpisodeTitleMissing)?);

		let string_pub_date = rss_item
			.pub_date()
			.ok_or(ParsingError::EpisodePubDateMissing)?;
		let mut pub_date = DateTime::parse_from_rfc2822(string_pub_date)?
			.naive_local()
			.date();

		if let Some(date_extractor) = show.date_extractor() {
			if let Some((date, range)) = date_extractor.extract_date(&title) {
				pub_date = date;
				title.to_mut().replace_range(range, "");
			}
		}
		let title = Self::process_raw_title(title, show.regex_container());

		let enclosure_url = rss_item
			.enclosure()
			.ok_or(ParsingError::EpisodeEnclosureURLMissing)?
			.url()
			.into();

		let filename = Self::generate_filename(&show, &pub_date, &title);

		Ok(Episode {
			enclosure_url,
			filename,
		})
	}

	fn generate_filename(show: &Show, pub_date: &NaiveDate, title: &str) -> String {
		format!(
			"{} - {} - {}.mp3",
			show.title(),
			Self::formatted_string_for_date(pub_date),
			title
		)
	}

	fn process_raw_title<S: Into<String>>(raw_title: S, regex_cont: Rc<RegexContainer>) -> String {
		use std::iter::once;

		// This bizarreness brought to you by the fact that [T; 2]'s iterator yields &T, but Map<T, _> yields T
		let default_patterns =
			once(regex_cont.leading_show_title_strip()).chain(once(&*EDGE_TRIM_REGEX));
		let custom_patterns = regex_cont.custom_episode_title_strips().iter();

		let strip_patterns = default_patterns.chain(custom_patterns);

		let mut processed_title: String = strip_patterns.fold(raw_title.into(), |title, reg| {
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

	fn formatted_string_for_date(date: &NaiveDate) -> impl std::fmt::Display {
		date.format("%F")
		// Year-month-day format (ISO 8601). Same as %Y-%m-%d. (https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
	}
}

#[cfg(test)]
mod tests {
	use super::super::{DateExtractionBuilder, DateFormat, ShowBuilder};
	use super::*;

	fn new_show(strip_patterns: Vec<&str>, date_format: Option<DateFormat>) -> Show {
		let de = date_format.map(|form| {
			DateExtractionBuilder::default()
				.format(form)
				.edge_strip_pattern_raw(r#"[\-\s]*"#)
				.build()
				.unwrap()
		});

		ShowBuilder::default()
			.title("FAKESHOW")
			.url("http://example.com/feed.rss")
			.title_strip_patterns(
				strip_patterns
					.into_iter()
					.map(|s| s.into())
					.collect::<Vec<String>>(),
			)
			.date_extraction(de)
			.build()
			.unwrap()
	}

	#[test]
	fn test_basic_stripping() {
		let show = new_show(vec![], None);
		let rc = show.regex_container();
		let raw_title =
			Episode::process_raw_title("FAKESHOW 666: We fought the devil - LIVE EPISODE ", rc);

		assert_eq!(raw_title, "666: We fought the devil - LIVE EPISODE")
	}

	#[test]
	fn test_regex_stripping() {
		let show = new_show(vec![r#"\s+-\s+LIVE EPISODE$"#], None);
		let rc = show.regex_container();
		let raw_title =
			Episode::process_raw_title("FAKESHOW 666: We fought the devil - LIVE EPISODE", rc);

		assert_eq!(raw_title, "666: We fought the devil")
	}

	#[test]
	fn test_american_conventional_date_extraction() {
		let show = new_show(vec![], Some(DateFormat::AmericanConventional));

		let enclosure = rss::EnclosureBuilder::default()
			.url("https://example.com/file.mp3")
			.build()
			.unwrap();

		let item = rss::ItemBuilder::default()
			.pub_date(Some("01 Jun 2016 14:31:46 -0700".into()))
			.title(Some("1/2/03 - Full Show".into()))
			.enclosure(Some(enclosure))
			.build()
			.unwrap();

		let ep = Episode::new(&show, item).unwrap();

		assert_eq!(ep.filename(), "FAKESHOW - 2003-01-02 - Full Show.mp3");
	}
}
