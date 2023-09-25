/// EventStatusType is an enumeration type whose instances represent several states that an Event may be in.
///
/// https://schema.org/EventStatusType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum EventStatusType {
    /// The event has been cancelled. If the event has multiple startDate values, all are assumed to be cancelled. Either startDate or previousStartDate may be used to specify the event's cancelled date(s).
    ///
    /// https://schema.org/EventCancelled
    EventCancelled,
    /// Indicates that the event was changed to allow online participation. See [[eventAttendanceMode]] for specifics of whether it is now fully or partially online.
    ///
    /// https://schema.org/EventMovedOnline
    EventMovedOnline,
    /// The event has been postponed and no new date has been set. The event's previousStartDate should be set.
    ///
    /// https://schema.org/EventPostponed
    EventPostponed,
    /// The event has been rescheduled. The event's previousStartDate should be set to the old date and the startDate should be set to the event's new date. (If the event has been rescheduled multiple times, the previousStartDate property may be repeated.)
    ///
    /// https://schema.org/EventRescheduled
    EventRescheduled,
    /// The event is taking place or has taken place on the startDate as scheduled. Use of this value is optional, as it is assumed by default.
    ///
    /// https://schema.org/EventScheduled
    EventScheduled,
}
