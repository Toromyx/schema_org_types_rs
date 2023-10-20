use super::*;
/// <https://schema.org/recommendedIntake>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
