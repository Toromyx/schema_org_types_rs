use super::*;
/// A CSS selector, e.g. of a [[SpeakableSpecification]] or [[WebPageElement]]. In the latter case, multiple matches within a page can constitute a single conceptual "Web page element".
///
/// https://schema.org/cssSelector
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
