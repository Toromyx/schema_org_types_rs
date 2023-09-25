use super::*;
/// Identifies that this [[Trip]] is a subTrip of another Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
///
/// <https://schema.org/partOfTrip>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfTripProperty {
    #[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
    Trip(Trip),
}
