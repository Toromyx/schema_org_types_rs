use super::*;
/// The station from which the train departs.
///
/// https://schema.org/departureStation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DepartureStationProperty {
    #[cfg(any(feature = "train-station-schema", feature = "general-schema-section"))]
    TrainStation(TrainStation),
}
