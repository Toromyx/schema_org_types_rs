use super::*;
/// A license document that applies to this content, typically indicated by URL.
///
/// https://schema.org/license
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LicenseProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}