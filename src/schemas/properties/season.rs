use super::*;
/// A season in a media series.
///
/// https://schema.org/season
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SeasonProperty {
    #[cfg(any(
        feature = "creative-work-season-schema",
        feature = "general-schema-section"
    ))]
    CreativeWorkSeason(CreativeWorkSeason),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
