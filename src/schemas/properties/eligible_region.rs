use super::*;
/// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.\n\nSee also [[ineligibleRegion]].
///
///
/// https://schema.org/eligibleRegion
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EligibleRegionProperty {
    #[cfg(any(feature = "geo-shape-schema", feature = "general-schema-section"))]
    GeoShape(GeoShape),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}