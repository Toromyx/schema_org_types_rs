use super::*;
/// <https://schema.org/evidenceLevel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
