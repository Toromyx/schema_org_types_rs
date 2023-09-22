use super::*;
/// The Event where the CreativeWork was recorded. The CreativeWork may capture all or part of the event.
///
/// https://schema.org/recordedAt
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecordedAtProperty {
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
}