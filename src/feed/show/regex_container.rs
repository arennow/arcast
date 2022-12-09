use super::{Clusions, Show};
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

	#[cfg(test)]
	pub fn has_only_default_title_strip(&self) -> bool {
		self.custom_episode_title_strips.is_empty() && self.clusions.is_none()
	}
}
