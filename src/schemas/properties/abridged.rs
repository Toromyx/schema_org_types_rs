use super::*;
/// Indicates whether the book is an abridged edition.
///
/// https://schema.org/abridged
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AbridgedProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
}
