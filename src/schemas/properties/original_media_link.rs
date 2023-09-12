use super::*;
/// Link to the page containing an original version of the content, or directly to an online copy of the original [[MediaObject]] content, e.g. video file.
///
/// https://schema.org/originalMediaLink
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OriginalMediaLinkProperty {
    #[cfg(any(feature = "media-object-schema", feature = "general-schema-section"))]
    MediaObject(MediaObject),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
    #[cfg(any(feature = "web-page-schema", feature = "general-schema-section"))]
    WebPage(WebPage),
}
