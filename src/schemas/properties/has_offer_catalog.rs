use super::*;
/// <https://schema.org/hasOfferCatalog>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasOfferCatalogProperty {
    #[cfg(any(
        any(feature = "offer-catalog-schema", feature = "general-schema-section"),
        doc
    ))]
    OfferCatalog(OfferCatalog),
}
