use super::*;
/// A dosing schedule for the drug for a given population, either observed, recommended, or maximum dose based on the type used.
///
/// https://schema.org/doseSchedule
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DoseScheduleProperty {
    #[cfg(any(
        feature = "dose-schedule-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DoseSchedule(DoseSchedule),
}
