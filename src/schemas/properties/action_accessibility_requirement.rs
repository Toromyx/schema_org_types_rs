use super::*;
/// A set of requirements that must be fulfilled in order to perform an Action. If more than one value is specified, fulfilling one set of requirements will allow the Action to be performed.
///
/// https://schema.org/actionAccessibilityRequirement
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ActionAccessibilityRequirementProperty {
    #[cfg(any(
        feature = "action-access-specification-schema",
        feature = "general-schema-section"
    ))]
    ActionAccessSpecification(ActionAccessSpecification),
}
