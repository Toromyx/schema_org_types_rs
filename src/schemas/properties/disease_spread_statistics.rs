use super::*;
/// <https://schema.org/diseaseSpreadStatistics>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DiseaseSpreadStatisticsProperty {
    #[cfg(any(
        any(feature = "dataset-schema", feature = "general-schema-section"),
        doc
    ))]
    Dataset(Dataset),
    #[cfg(any(
        any(feature = "observation-schema", feature = "pending-schema-section"),
        doc
    ))]
    Observation(Observation),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
    #[cfg(any(
        any(feature = "web-content-schema", feature = "pending-schema-section"),
        doc
    ))]
    WebContent(WebContent),
}
