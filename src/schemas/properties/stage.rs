use super::*;
/// The stage of the condition, if applicable.
///
/// https://schema.org/stage
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum StageProperty {
    #[cfg(any(
        any(
            feature = "medical-condition-stage-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalConditionStage(MedicalConditionStage),
}
