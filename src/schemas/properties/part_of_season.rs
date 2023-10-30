use super::*;
/// <https://schema.org/partOfSeason>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfSeasonProperty {
	#[cfg(any(
		any(
			feature = "creative-work-season-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	CreativeWorkSeason(CreativeWorkSeason),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
