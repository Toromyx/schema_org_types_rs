use super::*;
/// <https://schema.org/healthcareReportingData>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
