use super::*;
/// A child of the person.
///
/// <https://schema.org/children>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ChildrenProperty {
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
}
