use super::*;
/// Identifies a [[Trip]] that is a subTrip of this Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
///
/// https://schema.org/subTrip
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubTripProperty {
    #[cfg(any(feature = "trip-schema", feature = "general-schema-section"))]
    Trip(Trip),
}
