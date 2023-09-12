use super::*;
/// A [[HyperTocEntry]] can have a [[tocContinuation]] indicated, which is another [[HyperTocEntry]] that would be the default next item to play or render.
///
/// https://schema.org/tocContinuation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TocContinuationProperty {
    #[cfg(any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"))]
    HyperTocEntry(HyperTocEntry),
}
