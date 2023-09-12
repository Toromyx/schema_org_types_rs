use super::*;
/// Indicates data describing a hospital, e.g. a CDC [[CDCPMDRecord]] or as some kind of [[Dataset]].
///
/// https://schema.org/healthcareReportingData
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HealthcareReportingDataProperty {
    #[cfg(any(feature = "cdcpmd-record-schema", feature = "pending-schema-section"))]
    CdcpmdRecord(CdcpmdRecord),
    #[cfg(any(feature = "dataset-schema", feature = "general-schema-section"))]
    Dataset(Dataset),
}
