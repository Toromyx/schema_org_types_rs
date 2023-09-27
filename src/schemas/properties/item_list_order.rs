use super::*;
/// <https://schema.org/itemListOrder>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
