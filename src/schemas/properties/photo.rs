use super::*;
/// <https://schema.org/photo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PhotoProperty {
	#[cfg(any(
		any(feature = "image-object-schema", feature = "general-schema-section"),
		doc
	))]
	ImageObject(ImageObject),
	#[cfg(any(
		any(feature = "photograph-schema", feature = "general-schema-section"),
		doc
	))]
	Photograph(Photograph),
}
