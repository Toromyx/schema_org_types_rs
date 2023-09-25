use super::*;
/// Indicates the availability type of the game content associated with this action, such as whether it is a full version or a demo.
///
/// https://schema.org/gameAvailabilityType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GameAvailabilityTypeProperty {
    #[cfg(any(
        any(
            feature = "game-availability-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    GameAvailabilityEnumeration(GameAvailabilityEnumeration),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
