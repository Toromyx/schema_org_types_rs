use super::*;
/// <https://schema.org/arrivalBusStop>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
