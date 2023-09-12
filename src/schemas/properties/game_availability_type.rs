use super::*;
/// Indicates the availability type of the game content associated with this action, such as whether it is a full version or a demo.
///
/// https://schema.org/gameAvailabilityType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GameAvailabilityTypeProperty {
    #[cfg(any(
        feature = "game-availability-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    GameAvailabilityEnumeration(GameAvailabilityEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
