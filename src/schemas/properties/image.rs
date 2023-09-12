use super::*;
/// An image of the item. This can be a [[URL]] or a fully described [[ImageObject]].
///
/// https://schema.org/image
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ImageProperty {
    #[cfg(any(feature = "image-object-schema", feature = "general-schema-section"))]
    ImageObject(ImageObject),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
