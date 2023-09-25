use super::*;
/// The condition, complication, or symptom whose risk is being estimated.
///
/// <https://schema.org/estimatesRiskOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
