use super::Show;
use crate::feed::{Clusions, DateFormat, TitleHandling};
use chrono::NaiveDate;
use std::error::Error;

#[test]
fn test_parse_basic_config() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml"
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert!(show.title_strip_patterns().is_none());
	assert!(show.regex_container().has_only_default_title_strip());
	assert!(show.date_extraction().is_none());
	assert!(show.not_before_date().is_none());

	Ok(())
}

#[test]
fn test_parse_with_date_extraction() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"dateExtraction": {
				"format": "AmericanConventional"
			}
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert!(show.title_strip_patterns().is_none());
	assert!(show.regex_container().has_only_default_title_strip());
	assert_eq!(
		show.date_extraction().as_ref().map(|de| *de.format()),
		Some(DateFormat::AmericanConventional)
	);
	assert!(show.not_before_date().is_none());

	Ok(())
}

#[test]
fn test_parse_with_title_strip_pattern() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"titleStripPatterns": [
				"\\s*Episode\\s*\\d+:\\s*",
				"Bonus:\\s*"
			]
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert_eq!(show.title_strip_patterns().map(<[String]>::len), Some(2));
	assert_eq!(
		show.regex_container().custom_episode_title_strips().len(),
		2
	);
	assert!(show.date_extraction().is_none());
	assert!(show.not_before_date().is_none());

	Ok(())
}

#[test]
fn test_parse_strip_whole_title() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"stripWholeTitle": true
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert!(show.title_strip_patterns().is_none());
	assert!(matches!(show.title_handling(), TitleHandling::StripAll));
	assert!(show.regex_container().has_only_default_title_strip());
	assert!(show.date_extraction().is_none());
	assert!(show.not_before_date().is_none());

	Ok(())
}

#[test]
fn test_parse_conflicting_title_handling() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"titleStripPatterns": ["\\s*Episode\\s*\\d+:\\s*"],
			"stripWholeTitle": true
		}
		"#;

	assert!(matches!(
		serde_json::from_str(json),
		Result::<Show, _>::Err(_)
	));

	Ok(())
}

#[test]
fn test_parse_with_exclusion() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"exclusionPatterns": [
				"(?i)Best of"
			]
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert!(show.title_strip_patterns().is_none());
	assert!(matches!(
		show.regex_container().clusions(),
		Some(Clusions::Exclusion(_))
	));
	assert!(show.date_extraction().is_none());
	assert!(show.not_before_date().is_none());

	Ok(())
}

#[test]
fn test_parse_conflicting_clusions() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"exclusionPatterns": [
				"(?i)Best of"
			],
			"inclusionPatterns": [
				"(?i)Worst in"
			]
		}
		"#;

	assert!(matches!(
		serde_json::from_str(json),
		Result::<Show, _>::Err(_)
	));

	Ok(())
}

#[test]
fn test_parse_not_before_date() -> Result<(), Box<dyn Error>> {
	let json = r#"
		{
			"title": "Hard Pod",
			"url": "https://example.com/hardpod.xml",
			"notBefore": "2022-06-01"
		}
		"#;

	let show: Show = serde_json::from_str(json)?;
	assert_eq!(show.title(), "Hard Pod");
	assert_eq!(show.url(), "https://example.com/hardpod.xml");
	assert!(show.title_strip_patterns().is_none());
	assert!(show.regex_container().has_only_default_title_strip());
	assert!(show.date_extraction().is_none());
	assert_eq!(
		show.not_before_date(),
		Some(NaiveDate::from_ymd(2022, 6, 1))
	);

	Ok(())
}
