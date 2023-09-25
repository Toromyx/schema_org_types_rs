use super::*;
/// A pointer to another product (or multiple products) for which this product is an accessory or spare part.
///
/// <https://schema.org/isAccessoryOrSparePartFor>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsAccessoryOrSparePartForProperty {
    #[cfg(any(
        any(feature = "product-schema", feature = "general-schema-section"),
        doc
    ))]
    Product(Product),
}
