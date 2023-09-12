use super::*;
/// The condition, complication, or symptom whose risk is being estimated.
///
/// https://schema.org/estimatesRiskOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EstimatesRiskOfProperty {
    #[cfg(any(
        feature = "medical-entity-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEntity(MedicalEntity),
}
