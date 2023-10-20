use super::*;
/// <https://schema.org/hasMenuItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMenuItemProperty {
	#[cfg(any(
		any(feature = "menu-item-schema", feature = "general-schema-section"),
		doc
	))]
	MenuItem(MenuItem),
}
