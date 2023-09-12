use super::*;
/// Quantitative measure of the physiologic output of the exercise; also referred to as energy expenditure.
///
/// https://schema.org/workload
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WorkloadProperty {
    #[cfg(any(feature = "energy-schema", feature = "general-schema-section"))]
    Energy(Energy),
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
}
