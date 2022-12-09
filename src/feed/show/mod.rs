use super::date_format::*;
use crate::cache::Cache;
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Clusions<T> {
	Inclusion(Vec<T>),
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

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Show {
	title: String,
	url: String,

	#[builder(default)]
	title_strip_patterns: Vec<String>,

	#[builder(default)]
	regex_container: Cache<RegexContainer>,

	#[builder(default)]
	date_extraction: Option<DateExtraction>,

	#[builder(default)]
	raw_clusions: Option<Clusions<String>>,

	#[builder(default)]
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
		self.date_extraction
			.as_ref()
			.map(DateExtraction::date_extractor)
	}

	pub fn not_before_date(&self) -> Option<NaiveDate> {
		self.not_before_date
	}
}

mod date_extraction;
pub use date_extraction::*;

mod deserialization;

mod regex_container;
pub use regex_container::*;

#[cfg(test)]
mod tests;
