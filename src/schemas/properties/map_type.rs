use super::*;
/// Indicates the kind of Map, from the MapCategoryType Enumeration.
///
/// https://schema.org/mapType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MapTypeProperty {
    #[cfg(any(
        feature = "map-category-type-schema",
        feature = "general-schema-section"
    ))]
    MapCategoryType(MapCategoryType),
}
