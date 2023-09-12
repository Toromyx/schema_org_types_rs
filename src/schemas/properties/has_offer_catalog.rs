use super::*;
/// Indicates an OfferCatalog listing for this Organization, Person, or Service.
///
/// https://schema.org/hasOfferCatalog
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasOfferCatalogProperty {
    #[cfg(any(feature = "offer-catalog-schema", feature = "general-schema-section"))]
    OfferCatalog(OfferCatalog),
}
