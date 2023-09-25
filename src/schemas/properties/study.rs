use super::*;
/// A medical study or trial related to this entity.
///
/// <https://schema.org/study>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum StudyProperty {
    #[cfg(any(
        any(
            feature = "medical-study-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalStudy(MedicalStudy),
}
