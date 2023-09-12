use super::*;
/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication.
///
/// https://schema.org/userInteractionCount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum UserInteractionCountProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
}
