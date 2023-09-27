/// <https://schema.org/MedicalAudienceType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalAudienceType {
    /// <https://schema.org/Clinician>
    Clinician,
    /// <https://schema.org/MedicalResearcher>
    MedicalResearcher,
}
