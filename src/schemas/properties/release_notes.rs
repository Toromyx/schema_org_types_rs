use super::*;
/// Description of what changed in this version.
///
/// https://schema.org/releaseNotes
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReleaseNotesProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
