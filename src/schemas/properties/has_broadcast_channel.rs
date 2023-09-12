use super::*;
/// A broadcast channel of a broadcast service.
///
/// https://schema.org/hasBroadcastChannel
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasBroadcastChannelProperty {
    #[cfg(any(
        feature = "broadcast-channel-schema",
        feature = "general-schema-section"
    ))]
    BroadcastChannel(BroadcastChannel),
}
