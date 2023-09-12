use super::*;
/// A person that acts as performing member of a sports team; a player as opposed to a coach.
///
/// https://schema.org/athlete
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AthleteProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
