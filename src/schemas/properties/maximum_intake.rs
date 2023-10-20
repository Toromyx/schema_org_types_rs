use super::*;
/// <https://schema.org/maximumIntake>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
