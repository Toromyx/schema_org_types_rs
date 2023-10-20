/// <https://schema.org/ReservationStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
