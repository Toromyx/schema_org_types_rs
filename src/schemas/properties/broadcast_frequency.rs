use super::*;
/// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
///
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
