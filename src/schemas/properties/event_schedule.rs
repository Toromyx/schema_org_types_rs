use super::*;
/// Associates an [[Event]] with a [[Schedule]]. There are circumstances where it is preferable to share a schedule for a series of
/// repeating events rather than data on the individual events themselves. For example, a website or application might prefer to publish a schedule for a weekly
/// gym class rather than provide data on every event. A schedule could be processed by applications to add forthcoming events to a calendar. An [[Event]] that
/// is associated with a [[Schedule]] using this property should not have [[startDate]] or [[endDate]] properties. These are instead defined within the associated
/// [[Schedule]], this avoids any ambiguity for clients using the data. The property might have repeated values to specify different schedules, e.g. for different months
/// or seasons.
///
/// https://schema.org/eventSchedule
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EventScheduleProperty {
    #[cfg(any(
        any(feature = "schedule-schema", feature = "pending-schema-section"),
        doc
    ))]
    Schedule(Schedule),
}
