use super::*;
/// <https://schema.org/differentialDiagnosis>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DifferentialDiagnosisProperty {
	#[cfg(any(
		any(
			feature = "d-dx-element-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	DDxElement(DDxElement),
}
