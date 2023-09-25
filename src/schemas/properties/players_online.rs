use super::*;
/// Number of players on the server.
///
/// <https://schema.org/playersOnline>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PlayersOnlineProperty {
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
}
