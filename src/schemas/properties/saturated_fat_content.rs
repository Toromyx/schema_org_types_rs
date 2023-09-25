use super::*;
/// The number of grams of saturated fat.
///
/// <https://schema.org/saturatedFatContent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SaturatedFatContentProperty {
    #[cfg(any(any(feature = "mass-schema", feature = "general-schema-section"), doc))]
    Mass(Mass),
}
