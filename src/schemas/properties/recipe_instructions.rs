use super::*;
/// <https://schema.org/recipeInstructions>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecipeInstructionsProperty {
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(
		any(feature = "item-list-schema", feature = "general-schema-section"),
		doc
	))]
	ItemList(ItemList),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
