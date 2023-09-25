use super::*;
/// A Category code contained in this code set.
///
/// https://schema.org/hasCategoryCode
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasCategoryCodeProperty {
    #[cfg(any(
        any(feature = "category-code-schema", feature = "pending-schema-section"),
        doc
    ))]
    CategoryCode(CategoryCode),
}
