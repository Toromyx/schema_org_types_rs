/// Enumeration(s) for use with [[measurementMethod]].
///
/// <https://schema.org/MeasurementMethodEnum>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MeasurementMethodEnum {
    /// An example [[MeasurementMethodEnum]] (to remove when real enums are added).
    ///
    /// <https://schema.org/ExampleMeasurementMethodEnum>
    ExampleMeasurementMethodEnum,
}
