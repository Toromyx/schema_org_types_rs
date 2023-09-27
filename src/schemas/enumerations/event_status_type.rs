/// <https://schema.org/EventStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventStatusType {
    /// <https://schema.org/EventCancelled>
    EventCancelled,
    /// <https://schema.org/EventMovedOnline>
    EventMovedOnline,
    /// <https://schema.org/EventPostponed>
    EventPostponed,
    /// <https://schema.org/EventRescheduled>
    EventRescheduled,
    /// <https://schema.org/EventScheduled>
    EventScheduled,
}
