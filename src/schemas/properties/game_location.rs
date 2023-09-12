use super::*;
/// Real or fictional location of the game (or part of game).
///
/// https://schema.org/gameLocation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GameLocationProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
    #[cfg(any(feature = "postal-address-schema", feature = "general-schema-section"))]
    PostalAddress(PostalAddress),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
