use super::*;
/// <https://schema.org/coverageStartTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoverageStartTimeProperty {
	#[cfg(any(
		any(feature = "date-time-schema", feature = "general-schema-section"),
		doc
	))]
	DateTime(DateTime),
}
