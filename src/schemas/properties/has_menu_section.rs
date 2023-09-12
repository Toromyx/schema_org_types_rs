use super::*;
/// A subgrouping of the menu (by dishes, course, serving time period, etc.).
///
/// https://schema.org/hasMenuSection
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMenuSectionProperty {
    #[cfg(any(feature = "menu-section-schema", feature = "general-schema-section"))]
    MenuSection(MenuSection),
}
