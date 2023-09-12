use super::*;
/// The stop or station from which the bus arrives.
///
/// https://schema.org/arrivalBusStop
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArrivalBusStopProperty {
    #[cfg(any(feature = "bus-station-schema", feature = "general-schema-section"))]
    BusStation(BusStation),
    #[cfg(any(feature = "bus-stop-schema", feature = "general-schema-section"))]
    BusStop(BusStop),
}
