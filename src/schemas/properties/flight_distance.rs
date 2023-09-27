use super::*;
/// <https://schema.org/flightDistance>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FlightDistanceProperty {
    #[cfg(any(
        any(feature = "distance-schema", feature = "general-schema-section"),
        doc
    ))]
    Distance(Distance),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
