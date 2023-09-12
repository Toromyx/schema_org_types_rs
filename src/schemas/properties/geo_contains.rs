use super::*;
/// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
///
/// https://schema.org/geoContains
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoContainsProperty {
    #[cfg(any(
        feature = "geospatial-geometry-schema",
        feature = "pending-schema-section"
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
