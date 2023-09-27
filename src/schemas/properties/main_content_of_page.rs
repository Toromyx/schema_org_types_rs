use super::*;
/// <https://schema.org/mainContentOfPage>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MainContentOfPageProperty {
    #[cfg(any(
        any(
            feature = "web-page-element-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    WebPageElement(WebPageElement),
}
