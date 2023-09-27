use super::*;
/// <https://schema.org/acceptedAnswer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AcceptedAnswerProperty {
    #[cfg(any(
        any(feature = "answer-schema", feature = "general-schema-section"),
        doc
    ))]
    Answer(Answer),
    #[cfg(any(
        any(feature = "item-list-schema", feature = "general-schema-section"),
        doc
    ))]
    ItemList(ItemList),
}
