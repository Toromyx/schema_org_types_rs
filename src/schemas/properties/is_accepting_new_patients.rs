use super::*;
/// <https://schema.org/isAcceptingNewPatients>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsAcceptingNewPatientsProperty {
	#[cfg(any(
		any(feature = "boolean-schema", feature = "general-schema-section"),
		doc
	))]
	Boolean(Boolean),
}
