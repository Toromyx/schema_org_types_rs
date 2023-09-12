use super::*;
/// Collection, [fonds](https://en.wikipedia.org/wiki/Fonds), or item held, kept or maintained by an [[ArchiveOrganization]].
///
/// https://schema.org/archiveHeld
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArchiveHeldProperty {
    #[cfg(any(
        feature = "archive-component-schema",
        feature = "pending-schema-section"
    ))]
    ArchiveComponent(ArchiveComponent),
}
