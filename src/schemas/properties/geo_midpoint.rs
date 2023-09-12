use super::*;
/// Indicates the GeoCoordinates at the centre of a GeoShape, e.g. GeoCircle.
///
/// https://schema.org/geoMidpoint
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoMidpointProperty {
    #[cfg(any(feature = "geo-coordinates-schema", feature = "general-schema-section"))]
    GeoCoordinates(GeoCoordinates),
}
