use super::*;
/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
///
/// https://schema.org/interactionStatistic
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractionStatisticProperty {
    #[cfg(any(
        feature = "interaction-counter-schema",
        feature = "general-schema-section"
    ))]
    InteractionCounter(InteractionCounter),
}
