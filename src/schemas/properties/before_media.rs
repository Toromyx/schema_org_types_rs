use super::*;
/// <https://schema.org/beforeMedia>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BeforeMediaProperty {
    #[cfg(any(
        any(feature = "media-object-schema", feature = "general-schema-section"),
        doc
    ))]
    MediaObject(MediaObject),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
