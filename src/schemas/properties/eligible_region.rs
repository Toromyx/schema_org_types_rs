use super::*;
/// <https://schema.org/eligibleRegion>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EligibleRegionProperty {
    #[cfg(any(
        any(feature = "geo-shape-schema", feature = "general-schema-section"),
        doc
    ))]
    GeoShape(GeoShape),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
