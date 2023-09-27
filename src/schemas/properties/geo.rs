use super::*;
/// <https://schema.org/geo>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoProperty {
    #[cfg(any(
        any(feature = "geo-coordinates-schema", feature = "general-schema-section"),
        doc
    ))]
    GeoCoordinates(GeoCoordinates),
    #[cfg(any(
        any(feature = "geo-shape-schema", feature = "general-schema-section"),
        doc
    ))]
    GeoShape(GeoShape),
}
