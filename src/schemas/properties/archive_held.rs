use super::*;
/// <https://schema.org/archiveHeld>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
