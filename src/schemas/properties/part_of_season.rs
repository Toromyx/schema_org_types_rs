use super::*;
/// The season to which this episode belongs.
///
/// https://schema.org/partOfSeason
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfSeasonProperty {
    #[cfg(any(
        feature = "creative-work-season-schema",
        feature = "general-schema-section"
    ))]
    CreativeWorkSeason(CreativeWorkSeason),
}