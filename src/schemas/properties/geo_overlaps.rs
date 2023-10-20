use super::*;
/// <https://schema.org/geoOverlaps>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoOverlapsProperty {
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
