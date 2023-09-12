use super::*;
/// A sub property of participant. The sports team that participated on this action.
///
/// https://schema.org/sportsTeam
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SportsTeamProperty {
    #[cfg(any(feature = "sports-team-schema", feature = "general-schema-section"))]
    SportsTeam(SportsTeam),
}
