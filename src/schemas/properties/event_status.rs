use super::*;
/// <https://schema.org/eventStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
