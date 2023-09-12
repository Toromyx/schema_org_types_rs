use super::*;
/// This property is deprecated, alongside the UserInteraction types on which it depended.
///
/// https://schema.org/interactionCount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractionCountProperty {}
