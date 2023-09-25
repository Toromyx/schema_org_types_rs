use super::*;
/// A medical guideline related to this entity.
///
/// <https://schema.org/guideline>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GuidelineProperty {
    #[cfg(any(
        any(
            feature = "medical-guideline-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalGuideline(MedicalGuideline),
}
