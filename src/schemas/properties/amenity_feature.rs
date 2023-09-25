use super::*;
/// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
///
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
