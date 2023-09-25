use super::*;
/// Specifying the health condition(s) of a patient, medical study, or other target audience.
///
/// https://schema.org/healthCondition
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HealthConditionProperty {
    #[cfg(any(
        any(
            feature = "medical-condition-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalCondition(MedicalCondition),
}
