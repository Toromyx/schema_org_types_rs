use super::*;
/// The BroadcastService offered on this channel.
///
/// https://schema.org/providesBroadcastService
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ProvidesBroadcastServiceProperty {
    #[cfg(any(
        any(
            feature = "broadcast-service-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BroadcastService(BroadcastService),
}
