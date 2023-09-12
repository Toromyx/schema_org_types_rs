use super::*;
/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
///
/// https://schema.org/brand
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BrandProperty {
    #[cfg(any(feature = "brand-schema", feature = "general-schema-section"))]
    Brand(Brand),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}
