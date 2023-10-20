use super::*;
/// <https://schema.org/associatedDisease>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedDiseaseProperty {
	#[cfg(any(
		any(
			feature = "medical-condition-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalCondition(MedicalCondition),
	#[cfg(any(
		any(feature = "property-value-schema", feature = "general-schema-section"),
		doc
	))]
	PropertyValue(PropertyValue),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
