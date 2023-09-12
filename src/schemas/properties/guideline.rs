use super::*;
/// A medical guideline related to this entity.
///
/// https://schema.org/guideline
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GuidelineProperty {
    #[cfg(any(
        feature = "medical-guideline-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalGuideline(MedicalGuideline),
}
