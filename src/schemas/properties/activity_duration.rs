use super::*;
/// Length of time to engage in the activity.
///
/// https://schema.org/activityDuration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ActivityDurationProperty {
    #[cfg(any(feature = "duration-schema", feature = "general-schema-section"))]
    Duration(Duration),
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
}
