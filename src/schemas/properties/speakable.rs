use super::*;
/// <https://schema.org/speakable>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SpeakableProperty {
	#[cfg(any(
		any(
			feature = "speakable-specification-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	SpeakableSpecification(SpeakableSpecification),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
