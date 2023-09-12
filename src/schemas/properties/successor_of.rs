use super::*;
/// A pointer from a newer variant of a product  to its previous, often discontinued predecessor.
///
/// https://schema.org/successorOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuccessorOfProperty {
    #[cfg(any(feature = "product-model-schema", feature = "general-schema-section"))]
    ProductModel(ProductModel),
}
