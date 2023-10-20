use super::*;
/// <https://schema.org/drugClass>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DrugClassProperty {
	#[cfg(any(
		any(
			feature = "drug-class-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	DrugClass(DrugClass),
}
