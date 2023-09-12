use super::*;
/// The spatialCoverage of a CreativeWork indicates the place(s) which are the focus of the content. It is a subproperty of
/// contentLocation intended primarily for more technical and detailed materials. For example with a Dataset, it indicates
/// areas that the dataset describes: a dataset of New York weather would have spatialCoverage which was the place: the state of New York.
///
/// https://schema.org/spatialCoverage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SpatialCoverageProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
