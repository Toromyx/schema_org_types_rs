use super::*;
/// A media object that encodes this CreativeWork. This property is a synonym for encoding.
///
/// https://schema.org/associatedMedia
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedMediaProperty {
    #[cfg(any(feature = "media-object-schema", feature = "general-schema-section"))]
    MediaObject(MediaObject),
}
