use super::*;
/// Smaller compositions included in this work (e.g. a movement in a symphony).
///
/// https://schema.org/includedComposition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludedCompositionProperty {
    #[cfg(any(
        feature = "music-composition-schema",
        feature = "general-schema-section"
    ))]
    MusicComposition(MusicComposition),
}
