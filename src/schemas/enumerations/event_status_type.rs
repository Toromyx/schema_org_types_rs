/// <https://schema.org/EventStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
