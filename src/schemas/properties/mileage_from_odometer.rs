use super::*;
/// The total distance travelled by the particular vehicle since its initial production, as read from its odometer.\n\nTypical unit code(s): KMT for kilometers, SMI for statute miles
///
/// https://schema.org/mileageFromOdometer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MileageFromOdometerProperty {
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
}
