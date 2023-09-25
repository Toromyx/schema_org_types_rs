use super::*;
/// Smaller compositions included in this work (e.g. a movement in a symphony).
///
/// https://schema.org/includedComposition
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludedCompositionProperty {
    #[cfg(any(
        any(
            feature = "music-composition-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MusicComposition(MusicComposition),
}
