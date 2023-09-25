use super::*;
/// The geographic area where the service is provided.
///
/// <https://schema.org/serviceArea>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServiceAreaProperty {
    #[cfg(any(
        any(
            feature = "administrative-area-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AdministrativeArea(AdministrativeArea),
    #[cfg(any(
        any(feature = "geo-shape-schema", feature = "general-schema-section"),
        doc
    ))]
    GeoShape(GeoShape),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
