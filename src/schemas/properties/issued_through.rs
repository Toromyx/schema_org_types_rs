use super::*;
/// The service through which the permit was granted.
///
/// https://schema.org/issuedThrough
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IssuedThroughProperty {
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
}
