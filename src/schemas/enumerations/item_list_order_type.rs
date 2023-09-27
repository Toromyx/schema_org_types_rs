/// <https://schema.org/ItemListOrderType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ItemListOrderType {
    /// <https://schema.org/ItemListOrderAscending>
    ItemListOrderAscending,
    /// <https://schema.org/ItemListOrderDescending>
    ItemListOrderDescending,
    /// <https://schema.org/ItemListUnordered>
    ItemListUnordered,
}
