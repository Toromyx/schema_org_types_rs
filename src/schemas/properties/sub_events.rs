use super::*;
/// Events that are a part of this event. For example, a conference event includes many presentations, each subEvents of the conference.
///
/// https://schema.org/subEvents
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubEventsProperty {
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
}
