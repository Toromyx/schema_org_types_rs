use super::*;
/// A contact point for a person or organization.
///
/// https://schema.org/contactPoint
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ContactPointProperty {
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
}
