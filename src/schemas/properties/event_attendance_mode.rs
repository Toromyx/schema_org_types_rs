use super::*;
/// <https://schema.org/eventAttendanceMode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EventAttendanceModeProperty {
    #[cfg(any(
        any(
            feature = "event-attendance-mode-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EventAttendanceModeEnumeration(EventAttendanceModeEnumeration),
}
