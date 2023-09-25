use super::*;
/// Indicates the status of drug prescription, e.g. local catalogs classifications or whether the drug is available by prescription or over-the-counter, etc.
///
/// <https://schema.org/prescriptionStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PrescriptionStatusProperty {
    #[cfg(any(
        any(
            feature = "drug-prescription-status-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DrugPrescriptionStatus(DrugPrescriptionStatus),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
