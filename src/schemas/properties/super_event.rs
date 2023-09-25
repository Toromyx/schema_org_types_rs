use super::*;
/// An event that this event is a part of. For example, a collection of individual music performances might each have a music festival as their superEvent.
///
/// <https://schema.org/superEvent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SuperEventProperty {
    #[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
    Event(Event),
}
