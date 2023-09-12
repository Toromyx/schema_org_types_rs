use super::*;
/// Indicates whether this content is family friendly.
///
/// https://schema.org/isFamilyFriendly
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsFamilyFriendlyProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
}
