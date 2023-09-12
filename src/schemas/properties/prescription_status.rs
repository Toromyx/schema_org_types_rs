use super::*;
/// Indicates the status of drug prescription, e.g. local catalogs classifications or whether the drug is available by prescription or over-the-counter, etc.
///
/// https://schema.org/prescriptionStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PrescriptionStatusProperty {
    #[cfg(any(
        feature = "drug-prescription-status-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DrugPrescriptionStatus(DrugPrescriptionStatus),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
