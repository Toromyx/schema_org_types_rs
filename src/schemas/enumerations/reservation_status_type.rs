/// Enumerated status values for Reservation.
///
/// <https://schema.org/ReservationStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ReservationStatusType {
    /// The status for a previously confirmed reservation that is now cancelled.
    ///
    /// <https://schema.org/ReservationCancelled>
    ReservationCancelled,
    /// The status of a confirmed reservation.
    ///
    /// <https://schema.org/ReservationConfirmed>
    ReservationConfirmed,
    /// The status of a reservation on hold pending an update like credit card number or flight changes.
    ///
    /// <https://schema.org/ReservationHold>
    ReservationHold,
    /// The status of a reservation when a request has been sent, but not confirmed.
    ///
    /// <https://schema.org/ReservationPending>
    ReservationPending,
}
