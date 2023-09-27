use super::*;
/// <https://schema.org/maximumIntake>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MaximumIntakeProperty {
    #[cfg(any(
        any(
            feature = "maximum-dose-schedule-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MaximumDoseSchedule(MaximumDoseSchedule),
}
