use super::*;
/// <https://schema.org/diagram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DiagramProperty {
	#[cfg(any(
		any(feature = "image-object-schema", feature = "general-schema-section"),
		doc
	))]
	ImageObject(ImageObject),
}
