use super::*;
/// The geographic area where the service is provided.
///
/// https://schema.org/serviceArea
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServiceAreaProperty {
    #[cfg(any(
        feature = "administrative-area-schema",
        feature = "general-schema-section"
    ))]
    AdministrativeArea(AdministrativeArea),
    #[cfg(any(feature = "geo-shape-schema", feature = "general-schema-section"))]
    GeoShape(GeoShape),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
