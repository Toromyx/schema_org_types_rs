use super::*;
/// Type of ordering (e.g. Ascending, Descending, Unordered).
///
/// https://schema.org/itemListOrder
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ItemListOrderProperty {
    #[cfg(any(
        any(
            feature = "item-list-order-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ItemListOrderType(ItemListOrderType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
