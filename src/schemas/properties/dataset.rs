use super::*;
/// A dataset contained in this catalog.
///
/// https://schema.org/dataset
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DatasetProperty {
    #[cfg(any(feature = "dataset-schema", feature = "general-schema-section"))]
    Dataset(Dataset),
}