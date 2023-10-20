use super::*;
/// <https://schema.org/actionAccessibilityRequirement>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ActionAccessibilityRequirementProperty {
	#[cfg(any(
		any(
			feature = "action-access-specification-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	ActionAccessSpecification(ActionAccessSpecification),
}
