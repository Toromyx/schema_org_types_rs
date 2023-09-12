use super::*;
/// A citation or reference to another creative work, such as another publication, web page, scholarly article, etc.
///
/// https://schema.org/citation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CitationProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
