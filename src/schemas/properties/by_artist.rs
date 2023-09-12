use super::*;
/// The artist that performed this album or recording.
///
/// https://schema.org/byArtist
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ByArtistProperty {
    #[cfg(any(feature = "music-group-schema", feature = "general-schema-section"))]
    MusicGroup(MusicGroup),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
