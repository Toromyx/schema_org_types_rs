use super::*;
/// <https://schema.org/sportsTeam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SportsTeamProperty {
	#[cfg(any(
		any(feature = "sports-team-schema", feature = "general-schema-section"),
		doc
	))]
	SportsTeam(SportsTeam),
}
