use super::*;
/// <https://schema.org/owns>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OwnsProperty {
    #[cfg(any(
        any(feature = "ownership-info-schema", feature = "general-schema-section"),
        doc
    ))]
    OwnershipInfo(OwnershipInfo),
    #[cfg(any(
        any(feature = "product-schema", feature = "general-schema-section"),
        doc
    ))]
    Product(Product),
}
