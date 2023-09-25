use super::*;
/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
///
/// <https://schema.org/brand>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BrandProperty {
    #[cfg(any(any(feature = "brand-schema", feature = "general-schema-section"), doc))]
    Brand(Brand),
    #[cfg(any(
        any(feature = "organization-schema", feature = "general-schema-section"),
        doc
    ))]
    Organization(Organization),
}
