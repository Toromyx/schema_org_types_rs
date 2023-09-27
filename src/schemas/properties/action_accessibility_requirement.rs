use super::*;
/// <https://schema.org/actionAccessibilityRequirement>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
