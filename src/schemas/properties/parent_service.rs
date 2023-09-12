use super::*;
/// A broadcast service to which the broadcast service may belong to such as regional variations of a national channel.
///
/// https://schema.org/parentService
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ParentServiceProperty {
    #[cfg(any(
        feature = "broadcast-service-schema",
        feature = "general-schema-section"
    ))]
    BroadcastService(BroadcastService),
}
