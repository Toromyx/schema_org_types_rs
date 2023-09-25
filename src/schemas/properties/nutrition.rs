use super::*;
/// Nutrition information about the recipe or menu item.
///
/// https://schema.org/nutrition
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NutritionProperty {
    #[cfg(any(
        any(
            feature = "nutrition-information-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    NutritionInformation(NutritionInformation),
}
