use super::*;
/// The most generic uni-directional social relation.
///
/// https://schema.org/follows
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FollowsProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}