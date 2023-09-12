use super::*;
/// Specifics about the observational study design (enumerated).
///
/// https://schema.org/studyDesign
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StudyDesignProperty {
    #[cfg(any(
        feature = "medical-observational-study-design-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalObservationalStudyDesign(MedicalObservationalStudyDesign),
}
