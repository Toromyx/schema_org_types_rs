use super::*;
/// Strength of evidence of the data used to formulate the guideline (enumerated).
///
/// <https://schema.org/evidenceLevel>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EvidenceLevelProperty {
    #[cfg(any(
        any(
            feature = "medical-evidence-level-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalEvidenceLevel(MedicalEvidenceLevel),
}
