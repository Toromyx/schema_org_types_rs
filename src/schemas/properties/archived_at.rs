use super::*;
/// <https://schema.org/archivedAt>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArchivedAtProperty {
	#[cfg(any(
		any(feature = "web-page-schema", feature = "general-schema-section"),
		doc
	))]
	WebPage(WebPage),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
