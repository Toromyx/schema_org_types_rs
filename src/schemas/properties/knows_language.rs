use super::*;
/// Of a [[Person]], and less typically of an [[Organization]], to indicate a known language. We do not distinguish skill levels or reading/writing/speaking/signing here. Use language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47).
///
/// https://schema.org/knowsLanguage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum KnowsLanguageProperty {
    #[cfg(any(feature = "language-schema", feature = "general-schema-section"))]
    Language(Language),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
