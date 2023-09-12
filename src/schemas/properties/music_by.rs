use super::*;
/// The composer of the soundtrack.
///
/// https://schema.org/musicBy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MusicByProperty {
    #[cfg(any(feature = "music-group-schema", feature = "general-schema-section"))]
    MusicGroup(MusicGroup),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
