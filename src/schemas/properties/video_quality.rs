use super::*;
/// The quality of the video.
///
/// https://schema.org/videoQuality
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VideoQualityProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}