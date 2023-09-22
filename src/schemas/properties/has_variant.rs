use super::*;
/// Indicates a [[Product]] that is a member of this [[ProductGroup]] (or [[ProductModel]]).
///
/// https://schema.org/hasVariant
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasVariantProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
}