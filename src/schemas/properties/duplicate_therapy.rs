use super::*;
/// <https://schema.org/duplicateTherapy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DuplicateTherapyProperty {
	#[cfg(any(
		any(
			feature = "medical-therapy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTherapy(MedicalTherapy),
}
