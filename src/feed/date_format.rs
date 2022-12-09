use chrono::prelude::*;
use regex::Regex;
use serde::Deserialize;
use std::ops::Deref;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
enum RefOrNot<'a, T> {
	Owned(T),
	Borrowed(&'a T),
}

impl<'a, T> Deref for RefOrNot<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		match self {
			RefOrNot::Owned(o) => o,
			RefOrNot::Borrowed(b) => b,
		}
	}
}

lazy_static! {
	static ref AMERICAN_CONVENTIONAL_DATE_FORMAT_REGEX: Regex =
		Regex::new(r#"(\d{1,2})[\-/](\d{1,2})[\-/](\d{2,4})"#).unwrap();
}

#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum DateFormat {
	AmericanConventional,
}

impl DateFormat {
	pub fn make_extractor(self, edge_strip_raw_pattern: Option<&str>) -> DateExtractor<'static> {
		let composed_pattern = Self::composite_pattern(self.pattern(), edge_strip_raw_pattern);

		DateExtractor::new(self, composed_pattern)
	}

	fn pattern(self) -> &'static Regex {
		use DateFormat::*;
		match self {
			AmericanConventional => &AMERICAN_CONVENTIONAL_DATE_FORMAT_REGEX,
		}
	}

	fn extract_date(
		self,
		string: &str,
		composed_pattern: &RefOrNot<Regex>,
	) -> Option<(NaiveDate, Range<usize>)> {
		use DateFormat::*;
		match self {
			AmericanConventional => Self::extract_date_american(string, composed_pattern),
		}
	}

	fn extract_date_american(
		string: &str,
		composed_pattern: &RefOrNot<Regex>,
	) -> Option<(NaiveDate, Range<usize>)> {
		let captures = composed_pattern.captures(string)?;
		if captures.len() != 4 {
			// The 0th capture is the whole match
			return None;
		}

		fn get_num_from_capture<N: FromStr>(cap: Option<regex::Match>) -> Option<N> {
			cap?.as_str().parse().ok()
		}

		let month = get_num_from_capture(captures.get(1))?;
		let day = get_num_from_capture(captures.get(2))?;
		let mut year = get_num_from_capture(captures.get(3))?;
		if year < 100 {
			// 17 -> 2017
			year += 2000;
		}

		let date = NaiveDate::from_ymd_opt(year, month, day)?;
		let range = captures.get(0)?.range();

		Some((date, range))
	}

	fn composite_pattern<'a>(
		base: &'a Regex,
		edge_strip_raw_pattern: Option<&str>,
	) -> RefOrNot<'a, Regex> {
		edge_strip_raw_pattern.map_or(RefOrNot::Borrowed(base), |esrp| {
			let new_raw_pattern = format!("{}{}{}", esrp, base.as_str(), esrp);
			let new_pattern = crate::feed::RegexContainer::compile_pattern(&new_raw_pattern);

			RefOrNot::Owned(new_pattern)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_american_conventional_no_strip() {
		let extractor = DateFormat::AmericanConventional.make_extractor(None);

		let (date, range) = extractor
			.extract_date("FAKESHOW - 7/4/19 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 11..17);

		let (date, range) = extractor
			.extract_date("FAKESHOW - 07/04/2019 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 11..21);
	}

	#[test]
	fn test_american_conventional_strip() {
		let extractor = DateFormat::AmericanConventional.make_extractor(Some(r#"[\-\s]*"#));

		let (date, range) = extractor
			.extract_date("FAKESHOW - 7/4/19 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 8..20);

		let (date, range) = extractor
			.extract_date("FAKESHOW - 07/04/2019 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 8..24);
	}
}

#[derive(Debug)]
pub struct DateExtractor<'a> {
	format: DateFormat,
	composed_pattern: RefOrNot<'a, Regex>,
}

impl<'a> DateExtractor<'a> {
	fn new(format: DateFormat, composed_pattern: RefOrNot<'a, Regex>) -> Self {
		Self {
			format,
			composed_pattern,
		}
	}

	pub fn extract_date(&self, string: &str) -> Option<(NaiveDate, Range<usize>)> {
		self.format.extract_date(string, &self.composed_pattern)
	}
}
