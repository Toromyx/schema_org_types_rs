use super::*;
/// The event being broadcast such as a sporting event or awards ceremony.
///
/// https://schema.org/broadcastOfEvent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BroadcastOfEventProperty {
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
}
