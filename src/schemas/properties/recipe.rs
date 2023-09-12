use super::*;
/// A sub property of instrument. The recipe/instructions used to perform the action.
///
/// https://schema.org/recipe
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecipeProperty {
    #[cfg(any(feature = "recipe-schema", feature = "general-schema-section"))]
    Recipe(Recipe),
}
