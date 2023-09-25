use super::*;
/// The event being broadcast such as a sporting event or awards ceremony.
///
/// <https://schema.org/broadcastOfEvent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BroadcastOfEventProperty {
    #[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
    Event(Event),
}
