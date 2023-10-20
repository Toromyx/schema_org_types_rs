use super::*;
/// <https://schema.org/accessModeSufficient>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AccessModeSufficientProperty {
	#[cfg(any(
		any(feature = "item-list-schema", feature = "general-schema-section"),
		doc
	))]
	ItemList(ItemList),
}
