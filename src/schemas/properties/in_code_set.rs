use super::*;
/// <https://schema.org/inCodeSet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InCodeSetProperty {
	#[cfg(any(
		any(
			feature = "category-code-set-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	CategoryCodeSet(CategoryCodeSet),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
