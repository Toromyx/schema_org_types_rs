use super::*;
/// Represents spatial relations in which two geometries (or the places they represent) are topologically equal, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM). "Two geometries are topologically equal if their interiors intersect and no part of the interior or boundary of one geometry intersects the exterior of the other" (a symmetric relationship).
///
/// https://schema.org/geoEquals
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoEqualsProperty {
    #[cfg(any(
        feature = "geospatial-geometry-schema",
        feature = "pending-schema-section"
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
