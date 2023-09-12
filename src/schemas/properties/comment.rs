use super::*;
/// Comments, typically from users.
///
/// https://schema.org/comment
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CommentProperty {
    #[cfg(any(feature = "comment-schema", feature = "general-schema-section"))]
    Comment(Comment),
}
