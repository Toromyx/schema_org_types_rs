use super::*;
/// <https://schema.org/courseSchedule>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CourseScheduleProperty {
    #[cfg(any(
        any(feature = "schedule-schema", feature = "pending-schema-section"),
        doc
    ))]
    Schedule(Schedule),
}
