use super::*;
/// A subgrouping of the menu (by dishes, course, serving time period, etc.).
///
/// <https://schema.org/hasMenuSection>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasMenuSectionProperty {
    #[cfg(any(
        any(feature = "menu-section-schema", feature = "general-schema-section"),
        doc
    ))]
    MenuSection(MenuSection),
}
