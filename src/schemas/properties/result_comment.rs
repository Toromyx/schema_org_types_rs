use super::*;
/// <https://schema.org/resultComment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ResultCommentProperty {
	#[cfg(any(
		any(feature = "comment-schema", feature = "general-schema-section"),
		doc
	))]
	Comment(Comment),
}
