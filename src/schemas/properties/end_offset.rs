use super::*;
/// <https://schema.org/endOffset>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
