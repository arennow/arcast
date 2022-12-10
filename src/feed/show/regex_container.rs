use super::{Clusions, Show, TitleHandling};
use getset::Getters;
use regex::Regex;

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct RegexContainer {
	leading_show_title_strip: Regex,
	custom_episode_title_strips: Vec<Regex>,
	clusions: Option<Clusions<Regex>>,
}

impl From<&Show> for RegexContainer {
	fn from(show: &Show) -> Self {
		let escaped_show_title = regex::escape(show.title());
		let leading_show_title_strip =
			Regex::new(&format!(r#"{}[:\s]+"#, escaped_show_title)).unwrap();

		let custom_episode_title_strips = show
			.title_handling()
			.and_then(TitleHandling::strip_patterns)
			.map(|patterns| {
				patterns
					.iter()
					.map(|raw_str| RegexContainer::compile_pattern(raw_str))
					.collect()
			})
			.unwrap_or_default();

		let clusions = show.raw_clusions().as_ref().map(|clusions| {
			clusions.map(|string_vec| {
				string_vec
					.iter()
					.map(|s| RegexContainer::compile_pattern(s))
					.collect()
			})
		});

		RegexContainer {
			leading_show_title_strip,
			custom_episode_title_strips,
			clusions,
		}
	}
}

impl RegexContainer {
	pub fn compile_pattern(pattern: &str) -> Regex {
		Regex::new(pattern).expect("Bad Regex")
	}

	#[cfg(test)]
	pub fn has_only_default_title_strip(&self) -> bool {
		self.custom_episode_title_strips.is_empty() && self.clusions.is_none()
	}
}
