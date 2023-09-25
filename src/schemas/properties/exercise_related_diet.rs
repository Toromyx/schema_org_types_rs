use super::*;
/// A sub property of instrument. The diet used in this action.
///
/// <https://schema.org/exerciseRelatedDiet>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ExerciseRelatedDietProperty {
    #[cfg(any(
        any(feature = "diet-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Diet(Diet),
}
