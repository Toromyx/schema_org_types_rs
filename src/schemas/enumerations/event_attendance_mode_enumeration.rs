/// An EventAttendanceModeEnumeration value is one of potentially several modes of organising an event, relating to whether it is online or offline.
///
/// https://schema.org/EventAttendanceModeEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventAttendanceModeEnumeration {
    /// MixedEventAttendanceMode - an event that is conducted as a combination of both offline and online modes.
    ///
    /// https://schema.org/MixedEventAttendanceMode
    MixedEventAttendanceMode,
    /// OfflineEventAttendanceMode - an event that is primarily conducted offline.
    ///
    /// https://schema.org/OfflineEventAttendanceMode
    OfflineEventAttendanceMode,
    /// OnlineEventAttendanceMode - an event that is primarily conducted online.
    ///
    /// https://schema.org/OnlineEventAttendanceMode
    OnlineEventAttendanceMode,
}
