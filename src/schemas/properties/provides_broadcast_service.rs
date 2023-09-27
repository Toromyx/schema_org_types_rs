use super::*;
/// <https://schema.org/providesBroadcastService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
