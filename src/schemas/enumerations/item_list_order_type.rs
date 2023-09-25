/// Enumerated for values for itemListOrder for indicating how an ordered ItemList is organized.
///
/// <https://schema.org/ItemListOrderType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ItemListOrderType {
    /// An ItemList ordered with lower values listed first.
    ///
    /// <https://schema.org/ItemListOrderAscending>
    ItemListOrderAscending,
    /// An ItemList ordered with higher values listed first.
    ///
    /// <https://schema.org/ItemListOrderDescending>
    ItemListOrderDescending,
    /// An ItemList ordered with no explicit order.
    ///
    /// <https://schema.org/ItemListUnordered>
    ItemListUnordered,
}
