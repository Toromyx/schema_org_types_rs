use super::*;
/// <https://schema.org/geoMidpoint>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoMidpointProperty {
    #[cfg(any(
        any(feature = "geo-coordinates-schema", feature = "general-schema-section"),
        doc
    ))]
    GeoCoordinates(GeoCoordinates),
}
