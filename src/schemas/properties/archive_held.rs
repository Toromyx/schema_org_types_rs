use super::*;
/// Collection, [fonds](https://en.wikipedia.org/wiki/Fonds), or item held, kept or maintained by an [[ArchiveOrganization]].
///
/// <https://schema.org/archiveHeld>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ArchiveHeldProperty {
    #[cfg(any(
        any(
            feature = "archive-component-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ArchiveComponent(ArchiveComponent),
}
