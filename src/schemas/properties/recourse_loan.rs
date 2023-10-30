use super::*;
/// <https://schema.org/recourseLoan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecourseLoanProperty {
	#[cfg(any(
		any(feature = "boolean-schema", feature = "general-schema-section"),
		doc
	))]
	Boolean(Boolean),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
