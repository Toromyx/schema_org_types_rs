use super::*;
/// [[ArchiveOrganization]] that holds, keeps or maintains the [[ArchiveComponent]].
///
/// <https://schema.org/holdingArchive>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
