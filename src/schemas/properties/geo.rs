use super::*;
/// The geo coordinates of the place.
///
/// https://schema.org/geo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoProperty {
    #[cfg(any(feature = "geo-coordinates-schema", feature = "general-schema-section"))]
    GeoCoordinates(GeoCoordinates),
    #[cfg(any(feature = "geo-shape-schema", feature = "general-schema-section"))]
    GeoShape(GeoShape),
}
