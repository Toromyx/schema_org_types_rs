use super::*;
/// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
///
/// <https://schema.org/supersededBy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SupersededByProperty {
    #[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
    Class(Class),
    #[cfg(any(
        any(feature = "enumeration-schema", feature = "general-schema-section"),
        doc
    ))]
    Enumeration(Enumeration),
    #[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
    Property(Property),
}
