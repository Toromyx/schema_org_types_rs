use super::*;
/// <https://schema.org/medicalSpecialty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MedicalSpecialtyProperty {
    #[cfg(any(
        any(
            feature = "medical-specialty-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalSpecialty(MedicalSpecialty),
}
