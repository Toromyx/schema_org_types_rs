use super::*;
/// A photograph of this place.
///
/// https://schema.org/photo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PhotoProperty {
    #[cfg(any(feature = "image-object-schema", feature = "general-schema-section"))]
    ImageObject(ImageObject),
    #[cfg(any(feature = "photograph-schema", feature = "general-schema-section"))]
    Photograph(Photograph),
}