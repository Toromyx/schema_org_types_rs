use super::*;
/// <https://schema.org/requiresSubscription>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RequiresSubscriptionProperty {
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
    #[cfg(any(
        any(
            feature = "media-subscription-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MediaSubscription(MediaSubscription),
}
