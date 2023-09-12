use super::*;
/// An arrangement derived from the composition.
///
/// https://schema.org/musicArrangement
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MusicArrangementProperty {
    #[cfg(any(
        feature = "music-composition-schema",
        feature = "general-schema-section"
    ))]
    MusicComposition(MusicComposition),
}
