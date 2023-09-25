use super::*;
/// The current status of the reservation.
///
/// <https://schema.org/reservationStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReservationStatusProperty {
    #[cfg(any(
        any(
            feature = "reservation-status-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ReservationStatusType(ReservationStatusType),
}
