use chrono::prelude::*;
use regex::Regex;
use serde::Deserialize;
use std::ops::Range;
use std::str::FromStr;

lazy_static! {
	static ref AMERICAN_CONVENTIONAL_DATE_FORMAT_REGEX: Regex =
		Regex::new(r#"(\d{1,2})[\-/](\d{1,2})[\-/](\d{2,4})"#).unwrap();
}

#[derive(Debug, Deserialize, Clone)]
pub enum DateFormat {
	AmericanConventional,
}

impl DateFormat {
	pub fn extract_date(&self, string: &str) -> Option<(NaiveDate, Range<usize>)> {
		use DateFormat::*;
		match self {
			AmericanConventional => Self::extract_date_american(string),
		}
	}

	fn extract_date_american(string: &str) -> Option<(NaiveDate, Range<usize>)> {
		let captures = AMERICAN_CONVENTIONAL_DATE_FORMAT_REGEX.captures(string)?;
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_american_conventional() {
		let ac = DateFormat::AmericanConventional;

		let (date, range) = ac
			.extract_date("FAKESHOW - 7/4/19 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 11..17);

		let (date, range) = ac
			.extract_date("FAKESHOW - 07/04/2019 - How to raise the dead")
			.unwrap();

		assert_eq!(date, NaiveDate::from_ymd(2019, 7, 4));
		assert_eq!(range, 11..21);
	}
}
