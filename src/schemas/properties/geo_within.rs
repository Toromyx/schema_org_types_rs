use super::*;
/// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
///
/// https://schema.org/geoWithin
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoWithinProperty {
    #[cfg(any(
        feature = "geospatial-geometry-schema",
        feature = "pending-schema-section"
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
