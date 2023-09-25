use super::*;
/// A sub property of object. A question.
///
/// <https://schema.org/question>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum QuestionProperty {
    #[cfg(any(
        any(feature = "question-schema", feature = "general-schema-section"),
        doc
    ))]
    Question(Question),
}
