use super::*;
/// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
///
/// https://schema.org/supersededBy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SupersededByProperty {
    #[cfg(any(feature = "class-schema", feature = "meta-schema-section"))]
    Class(Class),
    #[cfg(any(feature = "enumeration-schema", feature = "general-schema-section"))]
    Enumeration(Enumeration),
    #[cfg(any(feature = "property-schema", feature = "meta-schema-section"))]
    Property(Property),
}
