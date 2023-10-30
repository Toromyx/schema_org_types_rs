use super::*;
/// <https://schema.org/mediaItemAppearance>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MediaItemAppearanceProperty {
	#[cfg(any(
		any(feature = "media-object-schema", feature = "general-schema-section"),
		doc
	))]
	MediaObject(MediaObject),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
