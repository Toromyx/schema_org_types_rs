use super::*;
/// <https://schema.org/bed>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BedProperty {
	#[cfg(any(
		any(feature = "bed-details-schema", feature = "general-schema-section"),
		doc
	))]
	BedDetails(BedDetails),
	#[cfg(any(
		any(feature = "bed-type-schema", feature = "general-schema-section"),
		doc
	))]
	BedType(BedType),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
