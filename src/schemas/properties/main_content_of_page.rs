use super::*;
/// Indicates if this web page element is the main subject of the page.
///
/// https://schema.org/mainContentOfPage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MainContentOfPageProperty {
    #[cfg(any(
        feature = "web-page-element-schema",
        feature = "general-schema-section"
    ))]
    WebPageElement(WebPageElement),
}
