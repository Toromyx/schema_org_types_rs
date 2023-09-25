use super::*;
/// The spatialCoverage of a CreativeWork indicates the place(s) which are the focus of the content. It is a subproperty of
/// contentLocation intended primarily for more technical and detailed materials. For example with a Dataset, it indicates
/// areas that the dataset describes: a dataset of New York weather would have spatialCoverage which was the place: the state of New York.
///
/// <https://schema.org/spatialCoverage>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SpatialCoverageProperty {
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
