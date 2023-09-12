use super::*;
/// A CSS selector, e.g. of a [[SpeakableSpecification]] or [[WebPageElement]]. In the latter case, multiple matches within a page can constitute a single conceptual "Web page element".
///
/// https://schema.org/cssSelector
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CssSelectorProperty {
    #[cfg(any(
        feature = "css-selector-type-schema",
        feature = "pending-schema-section"
    ))]
    CssSelectorType(CssSelectorType),
}
