use super::*;
/// For a [[NewsMediaOrganization]], a statement on coverage priorities, including any public agenda or stance on issues.
///
/// https://schema.org/missionCoveragePrioritiesPolicy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MissionCoveragePrioritiesPolicyProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
