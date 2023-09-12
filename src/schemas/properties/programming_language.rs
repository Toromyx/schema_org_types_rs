use super::*;
/// The computer programming language.
///
/// https://schema.org/programmingLanguage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProgrammingLanguageProperty {
    #[cfg(any(
        feature = "computer-language-schema",
        feature = "general-schema-section"
    ))]
    ComputerLanguage(ComputerLanguage),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
