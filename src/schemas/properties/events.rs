use super::*;
/// <https://schema.org/events>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EventsProperty {
	#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
	Event(Event),
}
