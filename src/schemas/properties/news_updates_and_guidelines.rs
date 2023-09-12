use super::*;
/// Indicates a page with news updates and guidelines. This could often be (but is not required to be) the main page containing [[SpecialAnnouncement]] markup on a site.
///
/// https://schema.org/newsUpdatesAndGuidelines
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NewsUpdatesAndGuidelinesProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
    #[cfg(any(feature = "web-content-schema", feature = "pending-schema-section"))]
    WebContent(WebContent),
}
