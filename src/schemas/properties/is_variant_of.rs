use super::*;
/// <https://schema.org/isVariantOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsVariantOfProperty {
    #[cfg(any(
        any(feature = "product-group-schema", feature = "pending-schema-section"),
        doc
    ))]
    ProductGroup(ProductGroup),
    #[cfg(any(
        any(feature = "product-model-schema", feature = "general-schema-section"),
        doc
    ))]
    ProductModel(ProductModel),
}
