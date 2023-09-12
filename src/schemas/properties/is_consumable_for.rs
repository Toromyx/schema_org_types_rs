use super::*;
/// A pointer to another product (or multiple products) for which this product is a consumable.
///
/// https://schema.org/isConsumableFor
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsConsumableForProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
}
