use super::*;
/// The time when the live blog will stop covering the Event. Note that coverage may continue after the Event concludes.
///
/// https://schema.org/coverageEndTime
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoverageEndTimeProperty {
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
}
