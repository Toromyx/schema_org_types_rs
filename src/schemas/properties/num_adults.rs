use super::*;
/// The number of adults staying in the unit.
///
/// https://schema.org/numAdults
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumAdultsProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
}