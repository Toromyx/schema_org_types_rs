use super::*;
/// <https://schema.org/cssSelector>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CssSelectorProperty {
    #[cfg(any(
        any(
            feature = "css-selector-type-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    CssSelectorType(CssSelectorType),
}
