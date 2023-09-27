use super::*;
/// <https://schema.org/recommendedIntake>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RecommendedIntakeProperty {
    #[cfg(any(
        any(
            feature = "recommended-dose-schedule-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    RecommendedDoseSchedule(RecommendedDoseSchedule),
}
