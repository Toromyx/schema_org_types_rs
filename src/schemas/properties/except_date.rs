use super::*;
/// <https://schema.org/exceptDate>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExceptDateProperty {
	#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
	Date(Date),
	#[cfg(any(
		any(feature = "date-time-schema", feature = "general-schema-section"),
		doc
	))]
	DateTime(DateTime),
}
