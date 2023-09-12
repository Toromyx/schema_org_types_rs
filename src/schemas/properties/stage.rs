use super::*;
/// The stage of the condition, if applicable.
///
/// https://schema.org/stage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StageProperty {
    #[cfg(any(
        feature = "medical-condition-stage-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalConditionStage(MedicalConditionStage),
}
