/// <https://schema.org/EventAttendanceModeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventAttendanceModeEnumeration {
	/// <https://schema.org/MixedEventAttendanceMode>
	MixedEventAttendanceMode,
	/// <https://schema.org/OfflineEventAttendanceMode>
	OfflineEventAttendanceMode,
	/// <https://schema.org/OnlineEventAttendanceMode>
	OnlineEventAttendanceMode,
}
