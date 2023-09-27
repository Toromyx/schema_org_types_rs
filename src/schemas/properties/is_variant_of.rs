use super::*;
/// <https://schema.org/isVariantOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
