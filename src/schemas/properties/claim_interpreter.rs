use super::*;
/// For a [[Claim]] interpreted from [[MediaObject]] content
/// sed to indicate a claim contained, implied or refined from the content of a [[MediaObject]].
///
/// https://schema.org/claimInterpreter
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ClaimInterpreterProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
