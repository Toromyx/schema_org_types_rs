use super::*;
/// <https://schema.org/suggestedGender>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SuggestedGenderProperty {
    #[cfg(any(
        any(feature = "gender-type-schema", feature = "general-schema-section"),
        doc
    ))]
    GenderType(GenderType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
