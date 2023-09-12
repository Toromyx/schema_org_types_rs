use super::*;
/// Specifying the health condition(s) of a patient, medical study, or other target audience.
///
/// https://schema.org/healthCondition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HealthConditionProperty {
    #[cfg(any(
        feature = "medical-condition-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalCondition(MedicalCondition),
}
