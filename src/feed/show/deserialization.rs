use super::{Clusions, Show, ShowBuilder};
use serde::{de, de::Visitor, Deserialize};

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "camelCase")]
enum Field {
	Title,
	Url,
	TitleStripPatterns,
	DateExtraction,
	InclusionPatterns,
	ExclusionPatterns,
	NotBefore,
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
				Field::TitleStripPatterns => {
					show_builder.title_strip_patterns(map.next_value::<Vec<_>>()?);
				}
				Field::DateExtraction => {
					show_builder.date_extraction(map.next_value::<Option<_>>()?);
				}
				Field::NotBefore => {
					show_builder.not_before_date(map.next_value::<Option<_>>()?);
				}
				Field::InclusionPatterns | Field::ExclusionPatterns => {
					if show_builder.has_raw_clusions() {
						return Err(de::Error::duplicate_field(
							"InclusionPatterns or ExclusionPatterns",
						));
					} else {
						let inside = map.next_value::<Vec<String>>()?;

						let clus = match key {
							Field::InclusionPatterns => Clusions::Inclusion(inside),
							Field::ExclusionPatterns => Clusions::Exclusion(inside),
							_ => unreachable!("the outer match should've prevented this"),
						};

						show_builder.raw_clusions(Some(dbg!(clus)));
					}
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
