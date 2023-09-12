use super::*;
/// An event that this event is a part of. For example, a collection of individual music performances might each have a music festival as their superEvent.
///
/// https://schema.org/superEvent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuperEventProperty {
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
}
