use super::*;
/// The eventAttendanceMode of an event indicates whether it occurs online, offline, or a mix.
///
/// https://schema.org/eventAttendanceMode
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
