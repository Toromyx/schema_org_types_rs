use super::*;
/// <https://schema.org/negativeNotes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
