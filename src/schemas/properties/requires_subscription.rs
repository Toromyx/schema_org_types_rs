use super::*;
/// <https://schema.org/requiresSubscription>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RequiresSubscriptionProperty {
    #[cfg(any(
        any(
            feature = "media-subscription-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MediaSubscription(MediaSubscription),
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
}
