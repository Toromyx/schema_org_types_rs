use super::*;
/// Indicates the kind of Map, from the MapCategoryType Enumeration.
///
/// <https://schema.org/mapType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MapTypeProperty {
    #[cfg(any(
        any(
            feature = "map-category-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MapCategoryType(MapCategoryType),
}
