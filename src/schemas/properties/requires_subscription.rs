use super::*;
/// Indicates if use of the media require a subscription  (either paid or free). Allowed values are ```true``` or ```false``` (note that an earlier version had 'yes', 'no').
///
/// https://schema.org/requiresSubscription
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RequiresSubscriptionProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
    #[cfg(any(
        feature = "media-subscription-schema",
        feature = "general-schema-section"
    ))]
    MediaSubscription(MediaSubscription),
}
