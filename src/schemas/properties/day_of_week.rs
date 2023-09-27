use super::*;
/// <https://schema.org/dayOfWeek>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DayOfWeekProperty {
    #[cfg(any(
        any(feature = "day-of-week-schema", feature = "general-schema-section"),
        doc
    ))]
    DayOfWeek(DayOfWeek),
}
