use super::*;
/// Identifies the denominator variable when an observation represents a ratio or percentage.
///
/// https://schema.org/measurementDenominator
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MeasurementDenominatorProperty {
    #[cfg(any(
        feature = "statistical-variable-schema",
        feature = "pending-schema-section"
    ))]
    StatisticalVariable(StatisticalVariable),
}
