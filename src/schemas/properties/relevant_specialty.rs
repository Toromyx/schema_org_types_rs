use super::*;
/// If applicable, a medical specialty in which this entity is relevant.
///
/// https://schema.org/relevantSpecialty
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RelevantSpecialtyProperty {
    #[cfg(any(
        any(
            feature = "medical-specialty-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalSpecialty(MedicalSpecialty),
}
