use super::*;
/// <https://schema.org/menuAddOn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MenuAddOnProperty {
    #[cfg(any(
        any(feature = "menu-item-schema", feature = "general-schema-section"),
        doc
    ))]
    MenuItem(MenuItem),
    #[cfg(any(
        any(feature = "menu-section-schema", feature = "general-schema-section"),
        doc
    ))]
    MenuSection(MenuSection),
}
