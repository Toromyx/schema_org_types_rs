use super::*;
/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
///
/// https://schema.org/interactionStatistic
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InteractionStatisticProperty {
    #[cfg(any(
        any(
            feature = "interaction-counter-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    InteractionCounter(InteractionCounter),
}
