use super::*;
/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
///
/// <https://schema.org/geoCoveredBy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoCoveredByProperty {
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
