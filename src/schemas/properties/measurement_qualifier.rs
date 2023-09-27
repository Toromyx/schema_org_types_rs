use super::*;
/// <https://schema.org/measurementQualifier>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MeasurementQualifierProperty {
    #[cfg(any(
        any(feature = "enumeration-schema", feature = "general-schema-section"),
        doc
    ))]
    Enumeration(Enumeration),
}
