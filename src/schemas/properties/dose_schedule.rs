use super::*;
/// A dosing schedule for the drug for a given population, either observed, recommended, or maximum dose based on the type used.
///
/// <https://schema.org/doseSchedule>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DoseScheduleProperty {
    #[cfg(any(
        any(
            feature = "dose-schedule-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DoseSchedule(DoseSchedule),
}
