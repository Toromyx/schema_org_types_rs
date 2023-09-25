use super::*;
/// A dataset contained in this catalog.
///
/// <https://schema.org/dataset>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DatasetProperty {
    #[cfg(any(
        any(feature = "dataset-schema", feature = "general-schema-section"),
        doc
    ))]
    Dataset(Dataset),
}
