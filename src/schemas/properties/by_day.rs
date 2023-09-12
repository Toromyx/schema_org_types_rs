use super::*;
/// Defines the day(s) of the week on which a recurring [[Event]] takes place. May be specified using either [[DayOfWeek]], or alternatively [[Text]] conforming to iCal's syntax for byDay recurrence rules.
///
/// https://schema.org/byDay
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ByDayProperty {
    #[cfg(any(feature = "day-of-week-schema", feature = "general-schema-section"))]
    DayOfWeek(DayOfWeek),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
