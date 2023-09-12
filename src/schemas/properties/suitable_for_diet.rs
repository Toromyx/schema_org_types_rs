use super::*;
/// Indicates a dietary restriction or guideline for which this recipe or menu item is suitable, e.g. diabetic, halal etc.
///
/// https://schema.org/suitableForDiet
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuitableForDietProperty {
    #[cfg(any(feature = "restricted-diet-schema", feature = "general-schema-section"))]
    RestrictedDiet(RestrictedDiet),
}
