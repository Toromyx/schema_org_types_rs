use super::*;
/// Defines the frequency at which [[Event]]s will occur according to a schedule [[Schedule]]. The intervals between
/// events should be defined as a [[Duration]] of time.
///
/// https://schema.org/repeatFrequency
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RepeatFrequencyProperty {
    #[cfg(any(feature = "duration-schema", feature = "general-schema-section"))]
    Duration(Duration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
