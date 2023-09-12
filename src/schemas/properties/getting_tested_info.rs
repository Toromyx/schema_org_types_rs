use super::*;
/// Information about getting tested (for a [[MedicalCondition]]), e.g. in the context of a pandemic.
///
/// https://schema.org/gettingTestedInfo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GettingTestedInfoProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
    #[cfg(any(feature = "web-content-schema", feature = "pending-schema-section"))]
    WebContent(WebContent),
}
