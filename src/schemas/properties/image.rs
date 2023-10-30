use super::*;
/// <https://schema.org/image>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ImageProperty {
	#[cfg(any(
		any(feature = "image-object-schema", feature = "general-schema-section"),
		doc
	))]
	ImageObject(ImageObject),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
