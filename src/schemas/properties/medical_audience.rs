use super::*;
/// Medical audience for page.
///
/// <https://schema.org/medicalAudience>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MedicalAudienceProperty {
    #[cfg(any(
        any(
            feature = "medical-audience-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalAudience(MedicalAudience),
    #[cfg(any(
        any(
            feature = "medical-audience-type-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalAudienceType(MedicalAudienceType),
}
