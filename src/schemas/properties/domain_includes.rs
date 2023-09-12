use super::*;
/// Relates a property to a class that is (one of) the type(s) the property is expected to be used on.
///
/// https://schema.org/domainIncludes
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DomainIncludesProperty {
    #[cfg(any(feature = "class-schema", feature = "meta-schema-section"))]
    Class(Class),
}
