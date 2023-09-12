use super::*;
/// The medical conditions, treatments, etc. that are the subject of the guideline.
///
/// https://schema.org/guidelineSubject
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GuidelineSubjectProperty {
    #[cfg(any(
        feature = "medical-entity-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEntity(MedicalEntity),
}
