use super::*;
/// <https://schema.org/partOfTrip>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfTripProperty {
    #[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
    Trip(Trip),
}
