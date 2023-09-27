use super::*;
/// <https://schema.org/broadcastFrequency>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BroadcastFrequencyProperty {
    #[cfg(any(
        any(
            feature = "broadcast-frequency-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BroadcastFrequencySpecification(BroadcastFrequencySpecification),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
