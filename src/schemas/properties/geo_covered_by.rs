use super::*;
/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
///
/// https://schema.org/geoCoveredBy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoCoveredByProperty {
    #[cfg(any(
        feature = "geospatial-geometry-schema",
        feature = "pending-schema-section"
    ))]
    GeospatialGeometry(GeospatialGeometry),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
