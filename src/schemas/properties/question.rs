use super::*;
/// A sub property of object. A question.
///
/// https://schema.org/question
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum QuestionProperty {
    #[cfg(any(feature = "question-schema", feature = "general-schema-section"))]
    Question(Question),
}
