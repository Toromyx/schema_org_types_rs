use super::*;
/// <https://schema.org/diseaseSpreadStatistics>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
