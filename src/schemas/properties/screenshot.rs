use super::*;
/// A link to a screenshot image of the app.
///
/// https://schema.org/screenshot
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ScreenshotProperty {
    #[cfg(any(feature = "image-object-schema", feature = "general-schema-section"))]
    ImageObject(ImageObject),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
