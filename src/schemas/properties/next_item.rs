use super::*;
/// A link to the ListItem that follows the current one.
///
/// <https://schema.org/nextItem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NextItemProperty {
    #[cfg(any(
        any(feature = "list-item-schema", feature = "general-schema-section"),
        doc
    ))]
    ListItem(ListItem),
}
