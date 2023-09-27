use super::*;
/// <https://schema.org/saturatedFatContent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SaturatedFatContentProperty {
    #[cfg(any(any(feature = "mass-schema", feature = "general-schema-section"), doc))]
    Mass(Mass),
}
