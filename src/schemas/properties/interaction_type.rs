use super::*;
/// <https://schema.org/interactionType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InteractionTypeProperty {
    #[cfg(any(
        any(feature = "action-schema", feature = "general-schema-section"),
        doc
    ))]
    Action(Action),
}
