use super::{error::*, RegexContainer, Show};
use chrono::prelude::*;
use derive_getters::Getters;
use regex::Regex;
use std::borrow::Cow;
use std::ops::Range;
use std::rc::Rc;

lazy_static! {
	static ref EDGE_TRIM_REGEX: Regex = Regex::new(r#"^\s+|\s+$"#).unwrap();
	static ref ENCLOSURE_URL_FILE_EXTENSION_REGEX: Regex =
		Regex::new(r#"(?i)\.([a-z0-9]+)(?:\?.*?)?$"#).unwrap();
	static ref STANDARD_CHARACTER_REPLACEMENT_PAIRS: [(&'static str, &'static str); 1] =
		[(" ", " ")];
}

#[derive(Builder, Getters, Debug)]
#[builder(setter(into), pattern = "owned")]
pub struct Episode {
	enclosure_url: String,

	filename: String,
	#[getter(skip)]
	episode_name_range: Range<usize>,

	#[getter(skip)]
	pub_date: NaiveDate,
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

		let enclosure_url: String = rss_item
			.enclosure()
			.ok_or(ParsingError::EpisodeEnclosureURLMissing)?
			.url()
			.into();

		let filename_extension = Self::get_enclosure_extension(&enclosure_url);
		let (filename, episode_name_range) =
			Self::generate_filename(&show, &pub_date, &title, filename_extension);

		Ok(Episode {
			enclosure_url,
			filename,
			episode_name_range,
			pub_date,
		})
	}

	fn get_enclosure_extension(url: &str) -> &str {
		if let Some(captures) = (*ENCLOSURE_URL_FILE_EXTENSION_REGEX).captures(url) {
			if let Some(capt) = captures.get(1) {
				return capt.as_str();
			}
		}
		return "mp3";
	}

	fn generate_filename(
		show: &Show,
		pub_date: &NaiveDate,
		title: &str,
		extension: &str,
	) -> (String, Range<usize>) {
		let filename = format!(
			"{} - {} - {}.{}",
			show.title(),
			Self::formatted_string_for_date(pub_date),
			title,
			extension
		);

		let name_end_index = filename.len() - (extension.len() + 1); // +1 for the .

		(filename, 0..name_end_index)
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

		for (source, dest) in STANDARD_CHARACTER_REPLACEMENT_PAIRS.iter() {
			processed_title = processed_title.replace(source, dest);
		}

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

impl Episode {
	pub fn episode_name(&self) -> &str {
		&self.filename[self.episode_name_range.clone()]
	}

	pub fn pub_date(&self) -> NaiveDate {
		self.pub_date
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
			.not_before_date(None)
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
	fn test_standard_title_replacements() {
		let show = new_show(vec![], None);
		let rc = show.regex_container();
		let raw_title = Episode::process_raw_title("nbsp:  ;", rc);

		assert_eq!(raw_title, "nbsp:  ;")
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
	fn test_generate_filename() {
		let show = new_show(vec![], None);

		let pub_date = NaiveDate::from_ymd(2021, 2, 21);

		let (filename, ep_name_range) =
			Episode::generate_filename(&show, &pub_date, "This Great Ep!", "wavefile");

		let ep = EpisodeBuilder::default()
			.enclosure_url("https://example.com/file.mp3")
			.filename(&filename)
			.episode_name_range(ep_name_range.clone())
			.pub_date(pub_date)
			.build()
			.unwrap();

		assert_eq!(filename, "FAKESHOW - 2021-02-21 - This Great Ep!.wavefile");
		assert_eq!(ep_name_range, 0..38);
		assert_eq!(ep.episode_name(), "FAKESHOW - 2021-02-21 - This Great Ep!");
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

	#[test]
	fn test_enclosure_extension_extraction() {
		let mkvs = [
			"https://example.com/file.mkv",
			"https://example.com/file.mkv?",
			"https://example.com/file.mkv?query=thing",
		];

		for mkv in mkvs {
			assert_eq!(Episode::get_enclosure_extension(mkv), "mkv");
		}

		let mp3s = [
			"https://example.com/file.mp3",
			"https://example.com/file.mp3?",
			"https://example.com/file.mp3?query=thing",
			"https://example.com/file",
			"https://example.com/file?",
			"https://example.com/file?query=thing",
		];

		for mp3 in mp3s {
			assert_eq!(Episode::get_enclosure_extension(mp3), "mp3");
		}
	}
}
