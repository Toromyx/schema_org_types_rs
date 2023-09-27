use super::*;
/// <https://schema.org/supersededBy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
