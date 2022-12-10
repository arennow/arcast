use super::{Clusions, DateExtraction, RegexContainer, TitleHandling};
use crate::{cache::Cache, feed::DateExtractor};
use chrono::NaiveDate;
use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use std::rc::Rc;

#[derive(Debug, Getters, CopyGetters, Builder)]
#[builder(setter(into))]
#[getset(get = "pub")]
pub struct Show {
	title: String,
	url: String,

	#[builder(default)]
	#[getset(skip)]
	title_handling: Option<TitleHandling>,

	#[builder(default)]
	#[getset(skip)]
	regex_container: Cache<RegexContainer>,

	#[builder(default)]
	date_extraction: Option<DateExtraction>,

	#[builder(default)]
	raw_clusions: Option<Clusions<String>>,

	#[builder(default)]
	#[getset(skip)]
	#[get_copy = "pub"]
	not_before_date: Option<NaiveDate>,
}

impl Show {
	pub fn title_handling(&self) -> Option<&TitleHandling> {
		self.title_handling.as_ref()
	}

	#[cfg(test)]
	pub fn title_strip_patterns(&self) -> Option<&[String]> {
		self.title_handling()
			.and_then(TitleHandling::strip_patterns)
	}

	pub fn regex_container(&self) -> Rc<RegexContainer> {
		self.regex_container.get(|| RegexContainer::from(self))
	}

	pub fn date_extractor(&self) -> Option<Rc<DateExtractor<'static>>> {
		self.date_extraction
			.as_ref()
			.map(DateExtraction::date_extractor)
	}
}

impl ShowBuilder {
	pub fn has_raw_clusions(&self) -> bool {
		self.raw_clusions.is_some()
	}
}
