use super::*;
/// A pointer to another, somehow related product (or multiple products).
///
/// https://schema.org/isRelatedTo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsRelatedToProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
}
