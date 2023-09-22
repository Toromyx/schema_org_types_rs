use super::*;
/// The catalog number for the release.
///
/// https://schema.org/catalogNumber
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CatalogNumberProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}