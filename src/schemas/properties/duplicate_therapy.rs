use super::*;
/// A therapy that duplicates or overlaps this one.
///
/// https://schema.org/duplicateTherapy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DuplicateTherapyProperty {
    #[cfg(any(
        feature = "medical-therapy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTherapy(MedicalTherapy),
}
