use super::*;
/// The kind of aircraft (e.g., "Boeing 747").
///
/// https://schema.org/aircraft
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AircraftProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "vehicle-schema", feature = "general-schema-section"))]
    Vehicle(Vehicle),
}
