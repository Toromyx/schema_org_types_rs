use super::*;
/// <https://schema.org/amenityFeature>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AmenityFeatureProperty {
    #[cfg(any(
        any(
            feature = "location-feature-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    LocationFeatureSpecification(LocationFeatureSpecification),
}
