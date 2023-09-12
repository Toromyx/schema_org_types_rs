use super::*;
/// Information about public transport closures.
///
/// https://schema.org/publicTransportClosuresInfo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PublicTransportClosuresInfoProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
    #[cfg(any(feature = "web-content-schema", feature = "pending-schema-section"))]
    WebContent(WebContent),
}
