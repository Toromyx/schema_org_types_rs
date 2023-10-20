use super::*;
/// <https://schema.org/bookFormat>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BookFormatProperty {
	#[cfg(any(
		any(
			feature = "book-format-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	BookFormatType(BookFormatType),
}
