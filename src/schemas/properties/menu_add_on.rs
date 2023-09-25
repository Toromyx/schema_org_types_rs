use super::*;
/// Additional menu item(s) such as a side dish of salad or side order of fries that can be added to this menu item. Additionally it can be a menu section containing allowed add-on menu items for this menu item.
///
/// <https://schema.org/menuAddOn>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
