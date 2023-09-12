use super::*;
/// Additional menu item(s) such as a side dish of salad or side order of fries that can be added to this menu item. Additionally it can be a menu section containing allowed add-on menu items for this menu item.
///
/// https://schema.org/menuAddOn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MenuAddOnProperty {
    #[cfg(any(feature = "menu-item-schema", feature = "general-schema-section"))]
    MenuItem(MenuItem),
    #[cfg(any(feature = "menu-section-schema", feature = "general-schema-section"))]
    MenuSection(MenuSection),
}
