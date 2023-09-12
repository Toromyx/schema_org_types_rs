use super::*;
/// Provides additional qualification to an observation. For example, a GDP observation measures the Nominal value.
///
/// https://schema.org/measurementQualifier
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MeasurementQualifierProperty {
    #[cfg(any(feature = "enumeration-schema", feature = "general-schema-section"))]
    Enumeration(Enumeration),
}
