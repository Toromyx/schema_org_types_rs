use super::*;
/// The number of owners of the vehicle, including the current one.\n\nTypical unit code(s): C62
///
/// https://schema.org/numberOfPreviousOwners
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumberOfPreviousOwnersProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
}
