use super::*;
/// For an [[Organization]] (often but not necessarily a [[NewsMediaOrganization]]), a report on staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US) reports, or self-reported.
///
/// https://schema.org/diversityStaffingReport
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DiversityStaffingReportProperty {
    #[cfg(any(feature = "article-schema", feature = "general-schema-section"))]
    Article(Article),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
