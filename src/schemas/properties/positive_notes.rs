use super::*;
/// Provides positive considerations regarding something, for example product highlights or (alongside [[negativeNotes]]) pro/con lists for reviews.
///
/// In the case of a [[Review]], the property describes the [[itemReviewed]] from the perspective of the review; in the case of a [[Product]], the product itself is being described.
///
/// The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
///
/// https://schema.org/positiveNotes
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PositiveNotesProperty {
    #[cfg(any(feature = "item-list-schema", feature = "general-schema-section"))]
    ItemList(ItemList),
    #[cfg(any(feature = "list-item-schema", feature = "general-schema-section"))]
    ListItem(ListItem),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "web-content-schema", feature = "pending-schema-section"))]
    WebContent(WebContent),
}