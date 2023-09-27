use super::*;
/// <https://schema.org/occupationalCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OccupationalCategoryProperty {
    #[cfg(any(
        any(feature = "category-code-schema", feature = "pending-schema-section"),
        doc
    ))]
    CategoryCode(CategoryCode),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
