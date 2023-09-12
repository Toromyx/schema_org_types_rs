use super::*;
/// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
///
/// https://schema.org/amenityFeature
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AmenityFeatureProperty {
    #[cfg(any(
        feature = "location-feature-specification-schema",
        feature = "general-schema-section"
    ))]
    LocationFeatureSpecification(LocationFeatureSpecification),
}
