use super::*;
/// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.) use MediaObject and indicate the [[encodingFormat]].
///
/// https://schema.org/caption
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CaptionProperty {
    #[cfg(any(feature = "media-object-schema", feature = "general-schema-section"))]
    MediaObject(MediaObject),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
