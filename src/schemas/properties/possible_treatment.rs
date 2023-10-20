use super::*;
/// <https://schema.org/possibleTreatment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PossibleTreatmentProperty {
	#[cfg(any(
		any(
			feature = "medical-therapy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTherapy(MedicalTherapy),
}
