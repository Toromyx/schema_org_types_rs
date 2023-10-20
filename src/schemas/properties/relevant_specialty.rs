use super::*;
/// <https://schema.org/relevantSpecialty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelevantSpecialtyProperty {
	#[cfg(any(
		any(
			feature = "medical-specialty-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalSpecialty(MedicalSpecialty),
}
