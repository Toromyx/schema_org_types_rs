use super::*;
/// A possible serious complication and/or serious side effect of this therapy. Serious adverse outcomes include those that are life-threatening; result in death, disability, or permanent damage; require hospitalization or prolong existing hospitalization; cause congenital anomalies or birth defects; or jeopardize the patient and may require medical or surgical intervention to prevent one of the outcomes in this definition.
///
/// <https://schema.org/seriousAdverseOutcome>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SeriousAdverseOutcomeProperty {
    #[cfg(any(
        any(
            feature = "medical-entity-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalEntity(MedicalEntity),
}
