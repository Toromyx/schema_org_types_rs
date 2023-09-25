use super::*;
/// The parent of a question, answer or item in general.
///
/// https://schema.org/parentItem
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ParentItemProperty {
    #[cfg(any(
        any(feature = "comment-schema", feature = "general-schema-section"),
        doc
    ))]
    Comment(Comment),
}
