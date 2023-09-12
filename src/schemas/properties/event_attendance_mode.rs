use super::*;
/// The eventAttendanceMode of an event indicates whether it occurs online, offline, or a mix.
///
/// https://schema.org/eventAttendanceMode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EventAttendanceModeProperty {
    #[cfg(any(
        feature = "event-attendance-mode-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    EventAttendanceModeEnumeration(EventAttendanceModeEnumeration),
}
