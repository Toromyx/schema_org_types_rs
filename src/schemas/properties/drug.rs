use super::*;
/// <https://schema.org/drug>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DrugProperty {
	#[cfg(any(
		any(feature = "drug-schema", feature = "health-lifesci-schema-section"),
		doc
	))]
	Drug(Drug),
}
