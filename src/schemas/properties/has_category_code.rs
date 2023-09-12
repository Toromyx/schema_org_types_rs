use super::*;
/// A Category code contained in this code set.
///
/// https://schema.org/hasCategoryCode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasCategoryCodeProperty {
    #[cfg(any(feature = "category-code-schema", feature = "pending-schema-section"))]
    CategoryCode(CategoryCode),
}
