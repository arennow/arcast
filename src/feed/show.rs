use super::date_format::DateFormat;
use crate::cache::Cache;
use derive_getters::Getters;
use regex::Regex;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug, Builder, Getters)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct Show {
	title: String,
	url: String,

	#[serde(default)]
	title_strip_patterns: Vec<String>,

	#[serde(skip, default)]
	#[getter(skip)]
	regex_container: Cache<RegexContainer>,

	date_extraction_format: Option<DateFormat>,
}

impl Show {
	pub fn regex_container(&self) -> Rc<RegexContainer> {
		self.regex_container.get(|| RegexContainer::from(self))
	}
}

#[derive(Debug, Clone, derive_getters::Getters)]
pub struct RegexContainer {
	leading_show_title_strip: Regex,
	custom_episode_title_strips: Vec<Regex>,
}

impl From<&Show> for RegexContainer {
	fn from(show: &Show) -> Self {
		RegexContainer {
			leading_show_title_strip: Regex::new(&format!(r#"{}[:\s]+"#, show.title())).unwrap(),
			custom_episode_title_strips: show
				.title_strip_patterns
				.iter()
				.map(|raw_str| Regex::new(raw_str).expect("Bad Regex"))
				.collect(),
		}
	}
}
