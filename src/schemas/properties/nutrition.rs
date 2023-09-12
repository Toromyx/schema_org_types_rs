use super::*;
/// Nutrition information about the recipe or menu item.
///
/// https://schema.org/nutrition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NutritionProperty {
    #[cfg(any(
        feature = "nutrition-information-schema",
        feature = "general-schema-section"
    ))]
    NutritionInformation(NutritionInformation),
}
