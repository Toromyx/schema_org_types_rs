use super::*;
/// Statistical information about the spread of a disease, either as [[WebContent]], or
/// described directly as a [[Dataset]], or the specific [[Observation]]s in the dataset. When a [[WebContent]] URL is
/// provided, the page indicated might also contain more such markup.
///
/// https://schema.org/diseaseSpreadStatistics
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DiseaseSpreadStatisticsProperty {
    #[cfg(any(feature = "dataset-schema", feature = "general-schema-section"))]
    Dataset(Dataset),
    #[cfg(any(feature = "observation-schema", feature = "pending-schema-section"))]
    Observation(Observation),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
    #[cfg(any(feature = "web-content-schema", feature = "pending-schema-section"))]
    WebContent(WebContent),
}
