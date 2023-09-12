use super::*;
/// The service provided by this channel.
///
/// https://schema.org/providesService
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProvidesServiceProperty {
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
}
