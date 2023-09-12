use super::*;
/// A step in making the recipe, in the form of a single item (document, video, etc.) or an ordered list with HowToStep and/or HowToSection items.
///
/// https://schema.org/recipeInstructions
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecipeInstructionsProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "item-list-schema", feature = "general-schema-section"))]
    ItemList(ItemList),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
