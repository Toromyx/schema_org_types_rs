use super::*;
/// Provides negative considerations regarding something, most typically in pro/con lists for reviews (alongside [[positiveNotes]]). For symmetry
///
/// In the case of a [[Review]], the property describes the [[itemReviewed]] from the perspective of the review; in the case of a [[Product]], the product itself is being described. Since product descriptions
/// tend to emphasise positive claims, it may be relatively unusual to find [[negativeNotes]] used in this way. Nevertheless for the sake of symmetry, [[negativeNotes]] can be used on [[Product]].
///
/// The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most negative is at the beginning of the list).
///
/// <https://schema.org/negativeNotes>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NegativeNotesProperty {
    #[cfg(any(
        any(feature = "item-list-schema", feature = "general-schema-section"),
        doc
    ))]
    ItemList(ItemList),
    #[cfg(any(
        any(feature = "list-item-schema", feature = "general-schema-section"),
        doc
    ))]
    ListItem(ListItem),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(
        any(feature = "web-content-schema", feature = "pending-schema-section"),
        doc
    ))]
    WebContent(WebContent),
}
