use super::*;
/// A pointer from a previous, often discontinued variant of the product to its newer variant.
///
/// https://schema.org/predecessorOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PredecessorOfProperty {
    #[cfg(any(feature = "product-model-schema", feature = "general-schema-section"))]
    ProductModel(ProductModel),
}
