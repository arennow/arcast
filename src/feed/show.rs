use super::date_format::*;
use crate::cache::Cache;
use chrono::NaiveDate;
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

#[derive(Debug, Deserialize, Clone)]
pub enum Clusions<T> {
	#[serde(rename = "inclusionPatterns")]
	Inclusion(Vec<T>),

	#[serde(rename = "exclusionPatterns")]
	Exclusion(Vec<T>),
}

impl<T> Clusions<T> {
	fn map<R, F>(&self, f: F) -> Clusions<R>
	where
		F: Fn(&Vec<T>) -> Vec<R>,
	{
		use Clusions::*;
		match self {
			Inclusion(v) => Inclusion(f(v)),
			Exclusion(v) => Exclusion(f(v)),
		}
	}
}

#[derive(Deserialize, Debug, Builder)]
#[builder(setter(into), pattern = "owned")]
#[serde(rename_all = "camelCase")]
pub struct Show {
	title: String,
	url: String,

	#[serde(default)]
	title_strip_patterns: Vec<String>,

	#[serde(skip, default)]
	#[builder(default)]
	regex_container: Cache<RegexContainer>,

	date_extraction: Option<DateExtraction>,

	#[serde(flatten)]
	#[builder(default)]
	raw_clusions: Option<Clusions<String>>,

	#[serde(rename = "notBefore")]
	not_before_date: Option<NaiveDate>,
}

impl Show {
	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn url(&self) -> &str {
		&self.url
	}

	pub fn regex_container(&self) -> Rc<RegexContainer> {
		self.regex_container.get(|| RegexContainer::from(self))
	}

	pub fn date_extractor(&self) -> Option<Rc<DateExtractor<'static>>> {
		self.date_extraction.as_ref().map(|de| de.date_extractor())
	}

	pub fn not_before_date(&self) -> Option<NaiveDate> {
		self.not_before_date
	}
}

#[derive(Debug, Clone, derive_getters::Getters)]
pub struct RegexContainer {
	leading_show_title_strip: Regex,
	custom_episode_title_strips: Vec<Regex>,
	clusions: Option<Clusions<Regex>>,
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
				.map(|raw_str| RegexContainer::compile_pattern(raw_str))
				.collect(),
			clusions: show.raw_clusions.as_ref().map(|c| {
				c.map(|e| {
					e.iter()
						.map(|s| RegexContainer::compile_pattern(s))
						.collect()
				})
			}),
		}
	}
}

impl RegexContainer {
	pub fn compile_pattern(pattern: &str) -> Regex {
		Regex::new(pattern).expect("Bad Regex")
	}
}
