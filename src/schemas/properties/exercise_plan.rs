use super::*;
/// A sub property of instrument. The exercise plan used on this action.
///
/// https://schema.org/exercisePlan
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExercisePlanProperty {
    #[cfg(any(
        feature = "exercise-plan-schema",
        feature = "health-lifesci-schema-section"
    ))]
    ExercisePlan(ExercisePlan),
}
