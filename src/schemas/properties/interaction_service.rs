use super::*;
/// The WebSite or SoftwareApplication where the interactions took place.
///
/// https://schema.org/interactionService
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InteractionServiceProperty {
    #[cfg(any(
        any(
            feature = "software-application-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SoftwareApplication(SoftwareApplication),
    #[cfg(any(
        any(feature = "web-site-schema", feature = "general-schema-section"),
        doc
    ))]
    WebSite(WebSite),
}
