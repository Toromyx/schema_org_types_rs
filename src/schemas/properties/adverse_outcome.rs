use super::*;
/// <https://schema.org/adverseOutcome>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AdverseOutcomeProperty {
    #[cfg(any(
        any(
            feature = "medical-entity-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalEntity(MedicalEntity),
}
