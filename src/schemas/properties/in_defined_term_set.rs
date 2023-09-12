use super::*;
/// A [[DefinedTermSet]] that contains this term.
///
/// https://schema.org/inDefinedTermSet
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InDefinedTermSetProperty {
    #[cfg(any(
        feature = "defined-term-set-schema",
        feature = "pending-schema-section"
    ))]
    DefinedTermSet(DefinedTermSet),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
