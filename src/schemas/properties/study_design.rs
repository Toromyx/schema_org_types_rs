use super::*;
/// Specifics about the observational study design (enumerated).
///
/// https://schema.org/studyDesign
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum StudyDesignProperty {
    #[cfg(any(
        any(
            feature = "medical-observational-study-design-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalObservationalStudyDesign(MedicalObservationalStudyDesign),
}
