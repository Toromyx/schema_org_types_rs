use super::*;
/// <https://schema.org/eventSchedule>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EventScheduleProperty {
	#[cfg(any(
		any(feature = "schedule-schema", feature = "pending-schema-section"),
		doc
	))]
	Schedule(Schedule),
}
