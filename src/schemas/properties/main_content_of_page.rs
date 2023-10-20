use super::*;
/// <https://schema.org/mainContentOfPage>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
