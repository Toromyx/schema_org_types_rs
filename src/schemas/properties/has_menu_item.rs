use super::*;
/// A food or drink item contained in a menu or menu section.
///
/// https://schema.org/hasMenuItem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMenuItemProperty {
    #[cfg(any(feature = "menu-item-schema", feature = "general-schema-section"))]
    MenuItem(MenuItem),
}
