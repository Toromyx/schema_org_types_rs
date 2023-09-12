use super::*;
/// A colleague of the person.
///
/// https://schema.org/colleague
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ColleagueProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
