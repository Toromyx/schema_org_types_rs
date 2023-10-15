use super::*;
/// <https://schema.org/description>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DescriptionProperty {
    #[cfg(any(
        any(feature = "text-object-schema", feature = "general-schema-section"),
        doc
    ))]
    TextObject(TextObject),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
