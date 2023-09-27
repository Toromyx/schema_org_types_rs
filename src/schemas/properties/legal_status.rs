use super::*;
/// <https://schema.org/legalStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
