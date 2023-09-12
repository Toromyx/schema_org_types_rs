use super::*;
/// The parent of a question, answer or item in general.
///
/// https://schema.org/parentItem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ParentItemProperty {
    #[cfg(any(feature = "comment-schema", feature = "general-schema-section"))]
    Comment(Comment),
}
