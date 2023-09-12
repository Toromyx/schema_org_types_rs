use super::*;
/// A media object representing the circumstances while performing this direction.
///
/// https://schema.org/duringMedia
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DuringMediaProperty {
    #[cfg(any(feature = "media-object-schema", feature = "general-schema-section"))]
    MediaObject(MediaObject),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
