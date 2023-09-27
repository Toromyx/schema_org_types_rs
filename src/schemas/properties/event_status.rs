use super::*;
/// <https://schema.org/eventStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EventStatusProperty {
    #[cfg(any(
        any(
            feature = "event-status-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    EventStatusType(EventStatusType),
}
