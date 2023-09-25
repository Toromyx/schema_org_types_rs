use super::*;
/// The organization owning or operating the broadcast service.
///
/// <https://schema.org/broadcaster>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BroadcasterProperty {
    #[cfg(any(
        any(feature = "organization-schema", feature = "general-schema-section"),
        doc
    ))]
    Organization(Organization),
}
