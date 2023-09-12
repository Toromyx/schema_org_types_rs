use super::*;
/// [[ArchiveOrganization]] that holds, keeps or maintains the [[ArchiveComponent]].
///
/// https://schema.org/holdingArchive
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HoldingArchiveProperty {
    #[cfg(any(
        feature = "archive-organization-schema",
        feature = "pending-schema-section"
    ))]
    ArchiveOrganization(ArchiveOrganization),
}
