use super::*;
/// <https://schema.org/answerExplanation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AnswerExplanationProperty {
	#[cfg(any(
		any(feature = "comment-schema", feature = "general-schema-section"),
		doc
	))]
	Comment(Comment),
	#[cfg(any(
		any(feature = "web-content-schema", feature = "pending-schema-section"),
		doc
	))]
	WebContent(WebContent),
}
