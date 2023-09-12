use super::*;
/// The date and place the work was first performed.
///
/// https://schema.org/firstPerformance
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FirstPerformanceProperty {
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
}
