use super::*;
/// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).)
///
/// <https://schema.org/geoDisjoint>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoDisjointProperty {
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
