use super::*;
/// An Event that is part of this event. For example, a conference event includes many presentations, each of which is a subEvent of the conference.
///
/// <https://schema.org/subEvent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SubEventProperty {
    #[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
    Event(Event),
}
