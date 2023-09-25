use super::*;
/// A sub property of instrument. The exercise plan used on this action.
///
/// <https://schema.org/exercisePlan>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
