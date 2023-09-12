use super::*;
/// A person that acts in a coaching role for a sports team.
///
/// https://schema.org/coach
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoachProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
