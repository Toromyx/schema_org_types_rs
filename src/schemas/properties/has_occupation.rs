use super::*;
/// <https://schema.org/hasOccupation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasOccupationProperty {
	#[cfg(any(
		any(feature = "occupation-schema", feature = "general-schema-section"),
		doc
	))]
	Occupation(Occupation),
}
