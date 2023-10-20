use super::*;
/// <https://schema.org/tocEntry>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TocEntryProperty {
	#[cfg(any(
		any(feature = "hyper-toc-entry-schema", feature = "pending-schema-section"),
		doc
	))]
	HyperTocEntry(HyperTocEntry),
}
