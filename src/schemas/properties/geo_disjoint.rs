use super::*;
/// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).)
///
/// https://schema.org/geoDisjoint
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoDisjointProperty {
    #[cfg(any(
        feature = "geospatial-geometry-schema",
        feature = "pending-schema-section"
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
