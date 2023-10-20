use super::*;
/// <https://schema.org/sourcedFrom>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SourcedFromProperty {
	#[cfg(any(
		any(
			feature = "brain-structure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	BrainStructure(BrainStructure),
}
