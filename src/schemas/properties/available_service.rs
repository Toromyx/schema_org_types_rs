use super::*;
/// <https://schema.org/availableService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableServiceProperty {
	#[cfg(any(
		any(
			feature = "medical-procedure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalProcedure(MedicalProcedure),
	#[cfg(any(
		any(
			feature = "medical-test-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTest(MedicalTest),
	#[cfg(any(
		any(
			feature = "medical-therapy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTherapy(MedicalTherapy),
}
