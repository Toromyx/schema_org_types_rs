use super::*;
/// Describes, in a [[MediaReview]] when dealing with [[DecontextualizedContent]], background information that can contribute to better interpretation of the [[MediaObject]].
///
/// <https://schema.org/originalMediaContextDescription>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OriginalMediaContextDescriptionProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
