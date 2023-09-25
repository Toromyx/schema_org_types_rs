use super::*;
/// Represents the length and pace of a course, expressed as a [[Schedule]].
///
/// https://schema.org/courseSchedule
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CourseScheduleProperty {
    #[cfg(any(
        any(feature = "schedule-schema", feature = "pending-schema-section"),
        doc
    ))]
    Schedule(Schedule),
}
