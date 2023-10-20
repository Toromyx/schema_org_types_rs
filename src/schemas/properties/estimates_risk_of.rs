use super::*;
/// <https://schema.org/estimatesRiskOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EstimatesRiskOfProperty {
	#[cfg(any(
		any(
			feature = "medical-entity-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalEntity(MedicalEntity),
}
