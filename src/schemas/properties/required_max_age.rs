use super::*;
/// Audiences defined by a person's maximum age.
///
/// https://schema.org/requiredMaxAge
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RequiredMaxAgeProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
}