use super::*;
/// The model of the product. Use with the URL of a ProductModel or a textual representation of the model identifier. The URL of the ProductModel can be from an external source. It is recommended to additionally provide strong product identifiers via the gtin8/gtin13/gtin14 and mpn properties.
///
/// https://schema.org/model
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModelProperty {
    #[cfg(any(feature = "product-model-schema", feature = "general-schema-section"))]
    ProductModel(ProductModel),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
