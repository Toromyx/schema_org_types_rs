use super::*;
/// The airport where the flight terminates.
///
/// <https://schema.org/arrivalAirport>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ArrivalAirportProperty {
    #[cfg(any(
        any(feature = "airport-schema", feature = "general-schema-section"),
        doc
    ))]
    Airport(Airport),
}
