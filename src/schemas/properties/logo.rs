use super::*;
/// An associated logo.
///
/// <https://schema.org/logo>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LogoProperty {
    #[cfg(any(
        any(feature = "image-object-schema", feature = "general-schema-section"),
        doc
    ))]
    ImageObject(ImageObject),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
