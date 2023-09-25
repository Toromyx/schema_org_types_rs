use super::*;
/// A pointer to another, functionally similar product (or multiple products).
///
/// <https://schema.org/isSimilarTo>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsSimilarToProperty {
    #[cfg(any(
        any(feature = "product-schema", feature = "general-schema-section"),
        doc
    ))]
    Product(Product),
    #[cfg(any(
        any(feature = "service-schema", feature = "general-schema-section"),
        doc
    ))]
    Service(Service),
}
