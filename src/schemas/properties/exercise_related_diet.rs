use super::*;
/// A sub property of instrument. The diet used in this action.
///
/// https://schema.org/exerciseRelatedDiet
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExerciseRelatedDietProperty {
    #[cfg(any(feature = "diet-schema", feature = "health-lifesci-schema-section"))]
    Diet(Diet),
}
