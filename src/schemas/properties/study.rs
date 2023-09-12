use super::*;
/// A medical study or trial related to this entity.
///
/// https://schema.org/study
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StudyProperty {
    #[cfg(any(
        feature = "medical-study-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalStudy(MedicalStudy),
}
