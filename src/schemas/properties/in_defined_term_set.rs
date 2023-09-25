use super::*;
/// A [[DefinedTermSet]] that contains this term.
///
/// https://schema.org/inDefinedTermSet
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InDefinedTermSetProperty {
    #[cfg(any(
        any(
            feature = "defined-term-set-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    DefinedTermSet(DefinedTermSet),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
