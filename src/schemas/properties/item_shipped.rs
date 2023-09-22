use super::*;
/// Item(s) being shipped.
///
/// https://schema.org/itemShipped
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemShippedProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
}