/// <https://schema.org/ReservationStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ReservationStatusType {
    /// <https://schema.org/ReservationCancelled>
    ReservationCancelled,
    /// <https://schema.org/ReservationConfirmed>
    ReservationConfirmed,
    /// <https://schema.org/ReservationHold>
    ReservationHold,
    /// <https://schema.org/ReservationPending>
    ReservationPending,
}
