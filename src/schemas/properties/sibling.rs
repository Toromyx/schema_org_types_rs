use super::*;
/// A sibling of the person.
///
/// https://schema.org/sibling
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SiblingProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}