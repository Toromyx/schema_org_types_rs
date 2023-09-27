use super::*;
/// <https://schema.org/legalStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegalStatusProperty {
    #[cfg(any(
        any(
            feature = "drug-legal-status-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DrugLegalStatus(DrugLegalStatus),
    #[cfg(any(
        any(
            feature = "medical-enumeration-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalEnumeration(MedicalEnumeration),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
