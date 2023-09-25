/// Level of evidence for a medical guideline. Enumerated type.
///
/// https://schema.org/MedicalEvidenceLevel
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalEvidenceLevel {
    /// Data derived from multiple randomized clinical trials or meta-analyses.
    ///
    /// https://schema.org/EvidenceLevelA
    EvidenceLevelA,
    /// Data derived from a single randomized trial, or nonrandomized studies.
    ///
    /// https://schema.org/EvidenceLevelB
    EvidenceLevelB,
    /// Only consensus opinion of experts, case studies, or standard-of-care.
    ///
    /// https://schema.org/EvidenceLevelC
    EvidenceLevelC,
}
