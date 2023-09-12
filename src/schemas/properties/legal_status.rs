use super::*;
/// The drug or supplement's legal status, including any controlled substance schedules that apply.
///
/// https://schema.org/legalStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegalStatusProperty {
    #[cfg(any(
        feature = "drug-legal-status-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DrugLegalStatus(DrugLegalStatus),
    #[cfg(any(
        feature = "medical-enumeration-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEnumeration(MedicalEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
