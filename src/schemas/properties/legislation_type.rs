use super::*;
/// <https://schema.org/legislationType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationTypeProperty {
    #[cfg(any(
        any(feature = "category-code-schema", feature = "pending-schema-section"),
        doc
    ))]
    CategoryCode(CategoryCode),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
