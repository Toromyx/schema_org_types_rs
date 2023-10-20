use super::*;
/// <https://schema.org/specialty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SpecialtyProperty {
	#[cfg(any(
		any(feature = "specialty-schema", feature = "general-schema-section"),
		doc
	))]
	Specialty(Specialty),
}
