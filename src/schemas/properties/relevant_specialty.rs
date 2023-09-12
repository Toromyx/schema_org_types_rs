use super::*;
/// If applicable, a medical specialty in which this entity is relevant.
///
/// https://schema.org/relevantSpecialty
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelevantSpecialtyProperty {
    #[cfg(any(
        feature = "medical-specialty-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalSpecialty(MedicalSpecialty),
}
