use super::*;
/// <https://schema.org/cssSelector>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CssSelectorProperty {
	#[cfg(any(
		any(
			feature = "css-selector-type-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	CssSelectorType(CssSelectorType),
}
