/// <https://schema.org/EventAttendanceModeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum EventAttendanceModeEnumeration {
    /// <https://schema.org/MixedEventAttendanceMode>
    MixedEventAttendanceMode,
    /// <https://schema.org/OfflineEventAttendanceMode>
    OfflineEventAttendanceMode,
    /// <https://schema.org/OnlineEventAttendanceMode>
    OnlineEventAttendanceMode,
}
