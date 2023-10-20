use super::*;
/// <https://schema.org/holdingArchive>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HoldingArchiveProperty {
	#[cfg(any(
		any(
			feature = "archive-organization-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	ArchiveOrganization(ArchiveOrganization),
}
