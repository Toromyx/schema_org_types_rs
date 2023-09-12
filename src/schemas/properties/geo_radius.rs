use super::*;
/// Indicates the approximate radius of a GeoCircle (metres unless indicated otherwise via Distance notation).
///
/// https://schema.org/geoRadius
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoRadiusProperty {
    #[cfg(any(feature = "distance-schema", feature = "general-schema-section"))]
    Distance(Distance),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
