/// <https://schema.org/MedicalEvidenceLevel>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalEvidenceLevel {
    /// <https://schema.org/EvidenceLevelA>
    EvidenceLevelA,
    /// <https://schema.org/EvidenceLevelB>
    EvidenceLevelB,
    /// <https://schema.org/EvidenceLevelC>
    EvidenceLevelC,
}
