use super::*;
/// The number of words in the text of the Article.
///
/// https://schema.org/wordCount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WordCountProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
}
