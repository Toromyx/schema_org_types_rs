/// Target audiences types for medical web pages. Enumerated type.
///
/// https://schema.org/MedicalAudienceType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalAudienceType {
    /// Medical clinicians, including practicing physicians and other medical professionals involved in clinical practice.
    ///
    /// https://schema.org/Clinician
    Clinician,
    /// Medical researchers.
    ///
    /// https://schema.org/MedicalResearcher
    MedicalResearcher,
}
