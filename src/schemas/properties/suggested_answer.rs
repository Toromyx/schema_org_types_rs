use super::*;
/// An answer (possibly one of several, possibly incorrect) to a Question, e.g. on a Question/Answer site.
///
/// https://schema.org/suggestedAnswer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuggestedAnswerProperty {
    #[cfg(any(feature = "answer-schema", feature = "general-schema-section"))]
    Answer(Answer),
    #[cfg(any(feature = "item-list-schema", feature = "general-schema-section"))]
    ItemList(ItemList),
}
