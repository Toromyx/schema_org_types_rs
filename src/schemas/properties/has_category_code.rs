use super::*;
/// <https://schema.org/hasCategoryCode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasCategoryCodeProperty {
	#[cfg(any(
		any(feature = "category-code-schema", feature = "pending-schema-section"),
		doc
	))]
	CategoryCode(CategoryCode),
}
