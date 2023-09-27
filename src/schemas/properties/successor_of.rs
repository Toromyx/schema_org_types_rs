use super::*;
/// <https://schema.org/successorOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SuccessorOfProperty {
    #[cfg(any(
        any(feature = "product-model-schema", feature = "general-schema-section"),
        doc
    ))]
    ProductModel(ProductModel),
}
