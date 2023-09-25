use super::*;
/// A media object representing the circumstances while performing this direction.
///
/// <https://schema.org/duringMedia>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DuringMediaProperty {
    #[cfg(any(
        any(feature = "media-object-schema", feature = "general-schema-section"),
        doc
    ))]
    MediaObject(MediaObject),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
