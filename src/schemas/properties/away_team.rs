use super::*;
/// <https://schema.org/awayTeam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AwayTeamProperty {
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
    #[cfg(any(
        any(feature = "sports-team-schema", feature = "general-schema-section"),
        doc
    ))]
    SportsTeam(SportsTeam),
}
