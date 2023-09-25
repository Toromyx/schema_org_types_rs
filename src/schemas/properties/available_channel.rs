use super::*;
/// A means of accessing the service (e.g. a phone bank, a web site, a location, etc.).
///
/// <https://schema.org/availableChannel>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AvailableChannelProperty {
    #[cfg(any(
        any(feature = "service-channel-schema", feature = "general-schema-section"),
        doc
    ))]
    ServiceChannel(ServiceChannel),
}
