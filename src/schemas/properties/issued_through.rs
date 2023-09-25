use super::*;
/// The service through which the permit was granted.
///
/// <https://schema.org/issuedThrough>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IssuedThroughProperty {
    #[cfg(any(
        any(feature = "service-schema", feature = "general-schema-section"),
        doc
    ))]
    Service(Service),
}
