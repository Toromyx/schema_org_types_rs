use super::*;
/// A link to the ListItem that precedes the current one.
///
/// https://schema.org/previousItem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PreviousItemProperty {
    #[cfg(any(feature = "list-item-schema", feature = "general-schema-section"))]
    ListItem(ListItem),
}
