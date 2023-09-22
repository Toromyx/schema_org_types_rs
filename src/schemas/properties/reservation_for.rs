use super::*;
/// The thing -- flight, event, restaurant, etc. being reserved.
///
/// https://schema.org/reservationFor
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReservationForProperty {
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
}