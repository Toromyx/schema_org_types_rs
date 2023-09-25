use super::*;
/// The station where the train trip ends.
///
/// https://schema.org/arrivalStation
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ArrivalStationProperty {
    #[cfg(any(
        any(feature = "train-station-schema", feature = "general-schema-section"),
        doc
    ))]
    TrainStation(TrainStation),
}
