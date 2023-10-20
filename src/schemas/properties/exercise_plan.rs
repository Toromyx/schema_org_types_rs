use super::*;
/// <https://schema.org/exercisePlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExercisePlanProperty {
	#[cfg(any(
		any(
			feature = "exercise-plan-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	ExercisePlan(ExercisePlan),
}
