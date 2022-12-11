use super::{Clusions, Show, ShowBuilder, TitleHandling};
use serde::{de, de::Visitor, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(field_identifier, rename_all = "camelCase")]
enum Field {
	Title,
	Url,
	DateExtraction,
	StripWholeTitle,
	TitleStripPatterns,
	InclusionPatterns,
	ExclusionPatterns,
	NotBefore,
}

fn assert_empty<'de, A: serde::de::MapAccess<'de>>(
	has_val: bool,
	fields: &[Field],
) -> Result<(), A::Error> {
	if !has_val {
		return Ok(());
	}

	let joined_fields = {
		let fields_count = fields.len();

		let mut joined_fields = String::new();

		for (i, field) in fields.iter().enumerate() {
			joined_fields.push_str(&format!("'{field:?}'"));
			joined_fields.push_str(match fields_count - i {
				1 => "",
				2 => " or ",
				_ => ", ",
			});
		}

		joined_fields
	};

	Err(de::Error::custom(format!(
		"Only one of {joined_fields} is allowed at a time"
	)))
}

struct ShowVisitor;

impl<'de> Visitor<'de> for ShowVisitor {
	type Value = Show;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a correctly formed show description")
	}

	fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
		let mut show_builder = ShowBuilder::default();

		while let Some(key) = map.next_key()? {
			match key {
				Field::Title => {
					show_builder.title(map.next_value::<String>()?);
				}
				Field::Url => {
					show_builder.url(map.next_value::<String>()?);
				}
				Field::DateExtraction => {
					show_builder.date_extraction(map.next_value::<Option<_>>()?);
				}
				Field::StripWholeTitle => {
					assert_empty::<A>(
						show_builder.has_title_handling(),
						&[Field::StripWholeTitle, Field::TitleStripPatterns],
					)?;

					if map.next_value()? {
						show_builder.title_handling(TitleHandling::StripAll);
					}
				}
				Field::TitleStripPatterns => {
					assert_empty::<A>(
						show_builder.has_title_handling(),
						&[Field::StripWholeTitle, Field::TitleStripPatterns],
					)?;
					show_builder.title_handling(TitleHandling::StripPatterns(map.next_value()?));
				}
				Field::NotBefore => {
					show_builder.not_before_date(map.next_value::<Option<_>>()?);
				}
				Field::InclusionPatterns | Field::ExclusionPatterns => {
					assert_empty::<A>(
						show_builder.has_raw_clusions(),
						&[Field::InclusionPatterns, Field::ExclusionPatterns],
					)?;

					let inside = map.next_value::<Vec<String>>()?;

					let clus = match key {
						Field::InclusionPatterns => Clusions::Inclusion(inside),
						Field::ExclusionPatterns => Clusions::Exclusion(inside),
						_ => unreachable!("the outer match should've prevented this"),
					};

					show_builder.raw_clusions(Some(clus));
				}
			};
		}

		let show = show_builder
			.build()
			.map_err(|e| de::Error::custom(e.to_string()))?;

		Ok(show)
	}
}

impl<'de> Deserialize<'de> for Show {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_map(ShowVisitor)
	}
}
