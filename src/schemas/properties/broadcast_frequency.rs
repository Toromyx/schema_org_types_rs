use super::*;
/// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
///
/// https://schema.org/broadcastFrequency
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BroadcastFrequencyProperty {
    #[cfg(any(
        feature = "broadcast-frequency-specification-schema",
        feature = "general-schema-section"
    ))]
    BroadcastFrequencySpecification(BroadcastFrequencySpecification),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
