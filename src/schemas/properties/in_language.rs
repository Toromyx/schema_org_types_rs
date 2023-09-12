use super::*;
/// The language of the content or performance or used in an action. Please use one of the language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47). See also [[availableLanguage]].
///
/// https://schema.org/inLanguage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InLanguageProperty {
    #[cfg(any(feature = "language-schema", feature = "general-schema-section"))]
    Language(Language),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
