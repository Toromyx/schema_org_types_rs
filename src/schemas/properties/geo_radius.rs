use super::*;
/// Indicates the approximate radius of a GeoCircle (metres unless indicated otherwise via Distance notation).
///
/// https://schema.org/geoRadius
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeoRadiusProperty {
    #[cfg(any(
        any(feature = "distance-schema", feature = "general-schema-section"),
        doc
    ))]
    Distance(Distance),
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
