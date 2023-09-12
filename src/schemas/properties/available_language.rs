use super::*;
/// A language someone may use with or at the item, service or place. Please use one of the language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47). See also [[inLanguage]].
///
/// https://schema.org/availableLanguage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableLanguageProperty {
    #[cfg(any(feature = "language-schema", feature = "general-schema-section"))]
    Language(Language),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
