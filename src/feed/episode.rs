use super::error::*;
use super::RegexContainer;
use super::Show;
use chrono::prelude::*;
use derive_getters::Getters;
use regex::Regex;
use std::rc::Rc;

lazy_static! {
	static ref EDGE_TRIM_REGEX: Regex = Regex::new(r#"^\s+|\s+$"#).unwrap();
}

#[derive(Builder, Getters)]
#[builder(setter(into))]
pub struct Episode {
	enclosure_url: String,
	filename: String,
}

impl Episode {
	pub fn new(show: &Show, rss_item: rss::Item) -> Result<Self, ParsingError> {
		let title = Self::process_raw_title(
			rss_item.title().ok_or(ParsingError::EpisodeTitleMissing)?,
			show.regex_container(),
		);

		let string_pub_date = rss_item
			.pub_date()
			.ok_or(ParsingError::EpisodePubDateMissing)?;
		let pub_date = DateTime::parse_from_rfc2822(string_pub_date)?
			.naive_local()
			.date();

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
	use super::super::ShowBuilder;
	use super::*;
	use crate::cache::Cache;

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
			.regex_container(Cache::default())
			.build()
			.unwrap()
	}

	#[test]
	fn test_basic_stripping() {
		let show = new_show(vec![]);
		let rc = show.regex_container();
		let raw_title =
			Episode::process_raw_title("FAKESHOW 666: We fought the devil - LIVE EPISODE ", rc);

		assert_eq!(raw_title, "666: We fought the devil - LIVE EPISODE")
	}

	#[test]
	fn test_regex_stripping() {
		let show = new_show(vec![r#"\s+-\s+LIVE EPISODE$"#]);
		let rc = show.regex_container();
		let raw_title =
			Episode::process_raw_title("FAKESHOW 666: We fought the devil - LIVE EPISODE", rc);

		assert_eq!(raw_title, "666: We fought the devil")
	}
}
