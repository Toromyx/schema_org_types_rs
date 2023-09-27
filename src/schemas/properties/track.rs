use super::*;
/// <https://schema.org/track>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TrackProperty {
    #[cfg(any(
        any(feature = "item-list-schema", feature = "general-schema-section"),
        doc
    ))]
    ItemList(ItemList),
    #[cfg(any(
        any(feature = "music-recording-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicRecording(MusicRecording),
}
