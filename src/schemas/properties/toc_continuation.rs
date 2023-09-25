use super::*;
/// A [[HyperTocEntry]] can have a [[tocContinuation]] indicated, which is another [[HyperTocEntry]] that would be the default next item to play or render.
///
/// https://schema.org/tocContinuation
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TocContinuationProperty {
    #[cfg(any(
        any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"),
        doc
    ))]
    HyperTocEntry(HyperTocEntry),
}
