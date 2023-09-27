use super::*;
/// <https://schema.org/arrivalBusStop>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ArrivalBusStopProperty {
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
