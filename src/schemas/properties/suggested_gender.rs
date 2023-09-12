use super::*;
/// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
///
/// https://schema.org/suggestedGender
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuggestedGenderProperty {
    #[cfg(any(feature = "gender-type-schema", feature = "general-schema-section"))]
    GenderType(GenderType),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
