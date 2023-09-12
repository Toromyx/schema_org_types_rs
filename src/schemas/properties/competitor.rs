use super::*;
/// A competitor in a sports event.
///
/// https://schema.org/competitor
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CompetitorProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
    #[cfg(any(feature = "sports-team-schema", feature = "general-schema-section"))]
    SportsTeam(SportsTeam),
}
