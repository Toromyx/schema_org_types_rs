use super::*;
/// The "spatial" property can be used in cases when more specific properties
/// (e.g. [[locationCreated]], [[spatialCoverage]], [[contentLocation]]) are not known to be appropriate.
///
/// <https://schema.org/spatial>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SpatialProperty {
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
