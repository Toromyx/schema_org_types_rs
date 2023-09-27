use super::*;
/// <https://schema.org/healthcareReportingData>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HealthcareReportingDataProperty {
    #[cfg(any(
        any(feature = "cdcpmd-record-schema", feature = "pending-schema-section"),
        doc
    ))]
    CdcpmdRecord(CdcpmdRecord),
    #[cfg(any(
        any(feature = "dataset-schema", feature = "general-schema-section"),
        doc
    ))]
    Dataset(Dataset),
}
