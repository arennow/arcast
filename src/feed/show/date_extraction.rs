use crate::{
	cache::Cache,
	feed::{DateExtractor, DateFormat},
};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Clone, Debug, Getters, Builder)]
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
