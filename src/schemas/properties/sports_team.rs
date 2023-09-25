use super::*;
/// A sub property of participant. The sports team that participated on this action.
///
/// <https://schema.org/sportsTeam>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SportsTeamProperty {
    #[cfg(any(
        any(feature = "sports-team-schema", feature = "general-schema-section"),
        doc
    ))]
    SportsTeam(SportsTeam),
}
