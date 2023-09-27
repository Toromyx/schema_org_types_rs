use super::*;
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
