use super::*;
/// Indicates a [[HyperTocEntry]] in a [[HyperToc]].
///
/// https://schema.org/tocEntry
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TocEntryProperty {
    #[cfg(any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"))]
    HyperTocEntry(HyperTocEntry),
}
