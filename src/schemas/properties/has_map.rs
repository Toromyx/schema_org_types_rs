use super::*;
/// A URL to a map of the place.
///
/// https://schema.org/hasMap
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMapProperty {
    #[cfg(any(feature = "map-schema", feature = "general-schema-section"))]
    Map(Map),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
