use super::*;
/// A sub property of result. The Comment created or sent as a result of this action.
///
/// https://schema.org/resultComment
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ResultCommentProperty {
    #[cfg(any(
        any(feature = "comment-schema", feature = "general-schema-section"),
        doc
    ))]
    Comment(Comment),
}
