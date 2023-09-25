use super::*;
/// The composer of the soundtrack.
///
/// <https://schema.org/musicBy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MusicByProperty {
    #[cfg(any(
        any(feature = "music-group-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicGroup(MusicGroup),
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
}
