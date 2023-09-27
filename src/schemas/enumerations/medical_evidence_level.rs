/// <https://schema.org/MedicalEvidenceLevel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalEvidenceLevel {
    /// <https://schema.org/EvidenceLevelA>
    EvidenceLevelA,
    /// <https://schema.org/EvidenceLevelB>
    EvidenceLevelB,
    /// <https://schema.org/EvidenceLevelC>
    EvidenceLevelC,
}
