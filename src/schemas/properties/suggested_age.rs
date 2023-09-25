use super::*;
/// The age or age range for the intended audience or person, for example 3-12 months for infants, 1-5 years for toddlers.
///
/// <https://schema.org/suggestedAge>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SuggestedAgeProperty {
    #[cfg(any(
        any(
            feature = "quantitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QuantitativeValue(QuantitativeValue),
}
