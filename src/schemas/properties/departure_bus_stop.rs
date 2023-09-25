use super::*;
/// The stop or station from which the bus departs.
///
/// <https://schema.org/departureBusStop>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DepartureBusStopProperty {
    #[cfg(any(
        any(feature = "bus-station-schema", feature = "general-schema-section"),
        doc
    ))]
    BusStation(BusStation),
    #[cfg(any(
        any(feature = "bus-stop-schema", feature = "general-schema-section"),
        doc
    ))]
    BusStop(BusStop),
}
