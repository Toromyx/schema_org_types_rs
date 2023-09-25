use super::*;
/// A broadcast service to which the broadcast service may belong to such as regional variations of a national channel.
///
/// https://schema.org/parentService
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ParentServiceProperty {
    #[cfg(any(
        any(
            feature = "broadcast-service-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BroadcastService(BroadcastService),
}
