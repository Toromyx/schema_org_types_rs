use super::*;
/// <https://schema.org/inDefinedTermSet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
