use super::*;
/// The computer programming language.
///
/// https://schema.org/programmingLanguage
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ProgrammingLanguageProperty {
    #[cfg(any(
        any(
            feature = "computer-language-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ComputerLanguage(ComputerLanguage),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
