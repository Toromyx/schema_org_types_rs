use super::*;
/// The end time of the clip expressed as the number of seconds from the beginning of the work.
///
/// <https://schema.org/endOffset>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EndOffsetProperty {
    #[cfg(any(
        any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"),
        doc
    ))]
    HyperTocEntry(HyperTocEntry),
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}
