use super::*;
/// A pointer to another, functionally similar product (or multiple products).
///
/// https://schema.org/isSimilarTo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsSimilarToProperty {
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
}
