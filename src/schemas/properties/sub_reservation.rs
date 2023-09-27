use super::*;
/// <https://schema.org/subReservation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SubReservationProperty {
    #[cfg(any(
        any(feature = "reservation-schema", feature = "general-schema-section"),
        doc
    ))]
    Reservation(Reservation),
}
