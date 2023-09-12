use super::*;
/// The distance of the flight.
///
/// https://schema.org/flightDistance
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FlightDistanceProperty {
    #[cfg(any(feature = "distance-schema", feature = "general-schema-section"))]
    Distance(Distance),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
