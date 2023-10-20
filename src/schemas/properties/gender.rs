use super::*;
/// <https://schema.org/gender>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GenderProperty {
	#[cfg(any(
		any(feature = "gender-type-schema", feature = "general-schema-section"),
		doc
	))]
	GenderType(GenderType),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
