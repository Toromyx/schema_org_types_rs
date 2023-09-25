use super::*;
/// A sub property of location. The sports event where this action occurred.
///
/// https://schema.org/sportsEvent
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SportsEventProperty {
    #[cfg(any(
        any(feature = "sports-event-schema", feature = "general-schema-section"),
        doc
    ))]
    SportsEvent(SportsEvent),
}
