use super::*;
/// <https://schema.org/departureStation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DepartureStationProperty {
    #[cfg(any(
        any(feature = "train-station-schema", feature = "general-schema-section"),
        doc
    ))]
    TrainStation(TrainStation),
}
