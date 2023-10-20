use super::*;
/// <https://schema.org/catalog>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CatalogProperty {
	#[cfg(any(
		any(feature = "data-catalog-schema", feature = "general-schema-section"),
		doc
	))]
	DataCatalog(DataCatalog),
}
