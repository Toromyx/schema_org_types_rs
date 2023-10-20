use super::*;
/// <https://schema.org/ratingCount>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RatingCountProperty {
	#[cfg(any(
		any(feature = "integer-schema", feature = "general-schema-section"),
		doc
	))]
	Integer(Integer),
}
