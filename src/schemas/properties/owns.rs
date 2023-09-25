use super::*;
/// Products owned by the organization or person.
///
/// <https://schema.org/owns>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
