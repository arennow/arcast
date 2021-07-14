use serde::Deserialize;

#[derive(Deserialize, Debug, Builder)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct Show {
	title: String,
	url: String,

	#[serde(default)]
	title_strip_patterns: Vec<String>,
}

impl Show {
	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn url(&self) -> &str {
		&self.url
	}

	pub fn title_strip_patterns(&self) -> &[String] {
		&self.title_strip_patterns
	}
}
