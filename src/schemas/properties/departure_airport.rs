use super::*;
/// The airport where the flight originates.
///
/// https://schema.org/departureAirport
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DepartureAirportProperty {
    #[cfg(any(feature = "airport-schema", feature = "general-schema-section"))]
    Airport(Airport),
}
