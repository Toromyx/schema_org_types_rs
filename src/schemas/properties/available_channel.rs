use super::*;
/// A means of accessing the service (e.g. a phone bank, a web site, a location, etc.).
///
/// https://schema.org/availableChannel
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableChannelProperty {
    #[cfg(any(feature = "service-channel-schema", feature = "general-schema-section"))]
    ServiceChannel(ServiceChannel),
}
