use super::*;
/// <https://schema.org/seasons>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
