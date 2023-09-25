use super::*;
/// Link to the page containing an original version of the content, or directly to an online copy of the original [[MediaObject]] content, e.g. video file.
///
/// https://schema.org/originalMediaLink
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OriginalMediaLinkProperty {
    #[cfg(any(
        any(feature = "media-object-schema", feature = "general-schema-section"),
        doc
    ))]
    MediaObject(MediaObject),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
    #[cfg(any(
        any(feature = "web-page-schema", feature = "general-schema-section"),
        doc
    ))]
    WebPage(WebPage),
}
