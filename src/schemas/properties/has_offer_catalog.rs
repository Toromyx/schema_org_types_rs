use super::*;
/// <https://schema.org/hasOfferCatalog>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasOfferCatalogProperty {
	#[cfg(any(
		any(feature = "offer-catalog-schema", feature = "general-schema-section"),
		doc
	))]
	OfferCatalog(OfferCatalog),
}
