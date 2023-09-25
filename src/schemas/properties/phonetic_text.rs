use super::*;
/// Representation of a text [[textValue]] using the specified [[speechToTextMarkup]]. For example the city name of Houston in IPA: /ˈhjuːstən/.
///
/// <https://schema.org/phoneticText>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PhoneticTextProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
