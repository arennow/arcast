#[derive(Debug, Clone)]
pub enum TitleHandling {
	StripPatterns(Vec<String>),
	// StripAll,
}

impl TitleHandling {
	#[cfg(test)]
	pub fn from_strip_patterns(patterns: impl IntoIterator<Item = impl Into<String>>) -> Self {
		Self::StripPatterns(patterns.into_iter().map(Into::into).collect())
	}

	pub fn strip_patterns(&self) -> Option<&[String]> {
		match self {
			Self::StripPatterns(v) => Some(v),
			// Self::StripAll => None,
		}
	}
}
