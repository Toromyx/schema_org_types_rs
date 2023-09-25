use super::*;
/// The day of the week for which these opening hours are valid.
///
/// <https://schema.org/dayOfWeek>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DayOfWeekProperty {
    #[cfg(any(
        any(feature = "day-of-week-schema", feature = "general-schema-section"),
        doc
    ))]
    DayOfWeek(DayOfWeek),
}
