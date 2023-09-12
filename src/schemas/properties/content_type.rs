use super::*;
/// The supported content type(s) for an EntryPoint response.
///
/// https://schema.org/contentType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ContentTypeProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
