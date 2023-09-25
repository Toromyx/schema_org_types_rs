use super::*;
/// The number of episodes in this season or series.
///
/// <https://schema.org/numberOfEpisodes>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NumberOfEpisodesProperty {
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
}
