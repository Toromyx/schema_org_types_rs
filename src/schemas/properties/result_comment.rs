use super::*;
/// A sub property of result. The Comment created or sent as a result of this action.
///
/// https://schema.org/resultComment
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ResultCommentProperty {
    #[cfg(any(feature = "comment-schema", feature = "general-schema-section"))]
    Comment(Comment),
}
