use super::*;
/// The WebSite or SoftwareApplication where the interactions took place.
///
/// https://schema.org/interactionService
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractionServiceProperty {
    #[cfg(any(
        feature = "software-application-schema",
        feature = "general-schema-section"
    ))]
    SoftwareApplication(SoftwareApplication),
    #[cfg(any(feature = "web-site-schema", feature = "general-schema-section"))]
    WebSite(WebSite),
}
