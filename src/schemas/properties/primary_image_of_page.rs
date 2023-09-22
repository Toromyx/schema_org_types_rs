use super::*;
/// Indicates the main image on the page.
///
/// https://schema.org/primaryImageOfPage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PrimaryImageOfPageProperty {
    #[cfg(any(feature = "image-object-schema", feature = "general-schema-section"))]
    ImageObject(ImageObject),
}