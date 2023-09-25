use super::*;
/// Indicates a [[HyperTocEntry]] in a [[HyperToc]].
///
/// https://schema.org/tocEntry
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TocEntryProperty {
    #[cfg(any(
        any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"),
        doc
    ))]
    HyperTocEntry(HyperTocEntry),
}
