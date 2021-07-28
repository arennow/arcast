use super::date_format::*;
use crate::cache::Cache;
use derive_getters::Getters;
use regex::Regex;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug, Getters, Builder)]
#[builder(setter(into), pattern = "owned")]
pub struct DateExtraction {
	format: DateFormat,

	#[getter(skip)]
	#[serde(rename = "edgeStripPattern")]
	#[builder(default, setter(into, strip_option))]
	edge_strip_pattern_raw: Option<String>,

	#[serde(skip, default)]
	#[getter(skip)]
	#[builder(default)]
	date_extractor: Cache<DateExtractor<'static>>,
}

impl DateExtraction {
	pub fn date_extractor(&self) -> Rc<DateExtractor<'static>> {
		let esrp = self.edge_strip_pattern_raw.as_ref().map(|s| &s[..]);
		self.date_extractor.get(|| self.format.make_extractor(esrp))
	}
}

#[derive(Deserialize, Debug, Builder, Getters)]
#[builder(setter(into), pattern = "owned")]
#[serde(rename_all = "camelCase")]
pub struct Show {
	title: String,
	url: String,

	#[serde(default)]
	#[getter(skip)]
	title_strip_patterns: Vec<String>,

	#[serde(skip, default)]
	#[getter(skip)]
	regex_container: Cache<RegexContainer>,

	#[getter(skip)]
	date_extraction: Option<DateExtraction>,
}

impl Show {
	pub fn regex_container(&self) -> Rc<RegexContainer> {
		self.regex_container.get(|| RegexContainer::from(self))
	}

	pub fn date_extractor(&self) -> Option<Rc<DateExtractor<'static>>> {
		self.date_extraction.as_ref().map(|de| de.date_extractor())
	}
}

#[derive(Debug, Clone, derive_getters::Getters)]
pub struct RegexContainer {
	leading_show_title_strip: Regex,
	custom_episode_title_strips: Vec<Regex>,
}

impl From<&Show> for RegexContainer {
	fn from(show: &Show) -> Self {
		let escaped_show_title = regex::escape(&show.title);

		RegexContainer {
			leading_show_title_strip: Regex::new(&format!(r#"{}[:\s]+"#, escaped_show_title))
				.unwrap(),
			custom_episode_title_strips: show
				.title_strip_patterns
				.iter()
				.map(|raw_str| Regex::new(raw_str).expect("Bad Regex"))
				.collect(),
		}
	}
}
