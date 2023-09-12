use super::*;
/// The current status of the reservation.
///
/// https://schema.org/reservationStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReservationStatusProperty {
    #[cfg(any(
        feature = "reservation-status-type-schema",
        feature = "general-schema-section"
    ))]
    ReservationStatusType(ReservationStatusType),
}
