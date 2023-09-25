use super::*;
/// The most generic bi-directional social/work relation.
///
/// <https://schema.org/knows>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum KnowsProperty {
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
}
