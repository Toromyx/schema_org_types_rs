use super::*;
/// A data catalog which contains this dataset (this property was previously 'catalog', preferred name is now 'includedInDataCatalog').
///
/// https://schema.org/includedDataCatalog
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludedDataCatalogProperty {
    #[cfg(any(feature = "data-catalog-schema", feature = "general-schema-section"))]
    DataCatalog(DataCatalog),
}
