use super::*;
/// Products owned by the organization or person.
///
/// https://schema.org/owns
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OwnsProperty {
    #[cfg(any(feature = "ownership-info-schema", feature = "general-schema-section"))]
    OwnershipInfo(OwnershipInfo),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
}
