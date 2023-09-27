use super::*;
/// <https://schema.org/speakable>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SpeakableProperty {
    #[cfg(any(
        any(
            feature = "speakable-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SpeakableSpecification(SpeakableSpecification),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
