use super::*;
/// <https://schema.org/diversityStaffingReport>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DiversityStaffingReportProperty {
    #[cfg(any(
        any(feature = "article-schema", feature = "general-schema-section"),
        doc
    ))]
    Article(Article),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
