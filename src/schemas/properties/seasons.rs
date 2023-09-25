use super::*;
/// A season in a media series.
///
/// <https://schema.org/seasons>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SeasonsProperty {
    #[cfg(any(
        any(
            feature = "creative-work-season-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    CreativeWorkSeason(CreativeWorkSeason),
}
