use super::*;
/// The season to which this episode belongs.
///
/// <https://schema.org/partOfSeason>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfSeasonProperty {
    #[cfg(any(
        any(
            feature = "creative-work-season-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    CreativeWorkSeason(CreativeWorkSeason),
}
