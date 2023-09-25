use super::*;
/// The kind of aircraft (e.g., "Boeing 747").
///
/// https://schema.org/aircraft
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AircraftProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(
        any(feature = "vehicle-schema", feature = "general-schema-section"),
        doc
    ))]
    Vehicle(Vehicle),
}
