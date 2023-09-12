use super::*;
/// Languages in which subtitles/captions are available, in [IETF BCP 47 standard format](http://tools.ietf.org/html/bcp47).
///
/// https://schema.org/subtitleLanguage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubtitleLanguageProperty {
    #[cfg(any(feature = "language-schema", feature = "general-schema-section"))]
    Language(Language),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
