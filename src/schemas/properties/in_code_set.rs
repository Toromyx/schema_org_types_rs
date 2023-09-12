use super::*;
/// A [[CategoryCodeSet]] that contains this category code.
///
/// https://schema.org/inCodeSet
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InCodeSetProperty {
    #[cfg(any(
        feature = "category-code-set-schema",
        feature = "pending-schema-section"
    ))]
    CategoryCodeSet(CategoryCodeSet),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
