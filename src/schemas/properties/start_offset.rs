use super::*;
/// The start time of the clip expressed as the number of seconds from the beginning of the work.
///
/// https://schema.org/startOffset
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StartOffsetProperty {
    #[cfg(any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"))]
    HyperTocEntry(HyperTocEntry),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}
