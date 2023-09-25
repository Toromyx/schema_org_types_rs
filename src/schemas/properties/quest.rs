use super::*;
/// The task that a player-controlled character, or group of characters may complete in order to gain a reward.
///
/// <https://schema.org/quest>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum QuestProperty {
    #[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
    Thing(Thing),
}
