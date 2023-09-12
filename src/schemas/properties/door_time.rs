use super::*;
/// The time admission will commence.
///
/// https://schema.org/doorTime
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DoorTimeProperty {
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
    #[cfg(any(feature = "time-schema", feature = "general-schema-section"))]
    Time(Time),
}
