use super::*;
/// Indicates whether this content is family friendly.
///
/// <https://schema.org/isFamilyFriendly>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsFamilyFriendlyProperty {
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
}
