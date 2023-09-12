use super::*;
/// How often one should break from the activity.
///
/// https://schema.org/restPeriods
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RestPeriodsProperty {
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
