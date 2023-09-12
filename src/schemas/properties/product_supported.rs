use super::*;
/// The product or service this support contact point is related to (such as product support for a particular product line). This can be a specific product or product line (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
///
/// https://schema.org/productSupported
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProductSupportedProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
