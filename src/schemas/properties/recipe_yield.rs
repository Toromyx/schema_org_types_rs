use super::*;
/// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
///
/// https://schema.org/recipeYield
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecipeYieldProperty {
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
