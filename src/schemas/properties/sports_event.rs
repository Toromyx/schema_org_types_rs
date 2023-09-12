use super::*;
/// A sub property of location. The sports event where this action occurred.
///
/// https://schema.org/sportsEvent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SportsEventProperty {
    #[cfg(any(feature = "sports-event-schema", feature = "general-schema-section"))]
    SportsEvent(SportsEvent),
}
