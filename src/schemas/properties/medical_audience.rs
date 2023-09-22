use super::*;
/// Medical audience for page.
///
/// https://schema.org/medicalAudience
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MedicalAudienceProperty {
    #[cfg(any(
        feature = "medical-audience-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalAudience(MedicalAudience),
    #[cfg(any(
        feature = "medical-audience-type-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalAudienceType(MedicalAudienceType),
}