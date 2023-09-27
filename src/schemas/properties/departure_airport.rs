use super::*;
/// <https://schema.org/departureAirport>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DepartureAirportProperty {
    #[cfg(any(
        any(feature = "airport-schema", feature = "general-schema-section"),
        doc
    ))]
    Airport(Airport),
}
