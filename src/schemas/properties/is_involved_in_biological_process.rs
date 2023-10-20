use super::*;
/// <https://schema.org/isInvolvedInBiologicalProcess>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsInvolvedInBiologicalProcessProperty {
	#[cfg(any(
		any(feature = "defined-term-schema", feature = "pending-schema-section"),
		doc
	))]
	DefinedTerm(DefinedTerm),
	#[cfg(any(
		any(feature = "property-value-schema", feature = "general-schema-section"),
		doc
	))]
	PropertyValue(PropertyValue),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
