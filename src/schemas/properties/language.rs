use super::*;
/// A sub property of instrument. The language used on this action.
///
/// <https://schema.org/language>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LanguageProperty {
    #[cfg(any(
        any(feature = "language-schema", feature = "general-schema-section"),
        doc
    ))]
    Language(Language),
}
