/// Target audiences types for medical web pages. Enumerated type.
///
/// <https://schema.org/MedicalAudienceType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalAudienceType {
    /// Medical clinicians, including practicing physicians and other medical professionals involved in clinical practice.
    ///
    /// <https://schema.org/Clinician>
    Clinician,
    /// Medical researchers.
    ///
    /// <https://schema.org/MedicalResearcher>
    MedicalResearcher,
}
