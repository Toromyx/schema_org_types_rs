use super::*;
/// <https://schema.org/language>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LanguageProperty {
	#[cfg(any(
		any(feature = "language-schema", feature = "general-schema-section"),
		doc
	))]
	Language(Language),
}
