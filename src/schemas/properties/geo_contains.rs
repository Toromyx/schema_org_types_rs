use super::*;
/// <https://schema.org/geoContains>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoContainsProperty {
    #[cfg(any(
        any(
            feature = "geospatial-geometry-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
