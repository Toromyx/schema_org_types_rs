use super::*;
/// Identifies the denominator variable when an observation represents a ratio or percentage.
///
/// https://schema.org/measurementDenominator
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MeasurementDenominatorProperty {
    #[cfg(any(
        any(
            feature = "statistical-variable-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    StatisticalVariable(StatisticalVariable),
}
