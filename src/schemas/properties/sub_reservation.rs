use super::*;
/// The individual reservations included in the package. Typically a repeated property.
///
/// https://schema.org/subReservation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubReservationProperty {
    #[cfg(any(feature = "reservation-schema", feature = "general-schema-section"))]
    Reservation(Reservation),
}
