use super::*;
/// The number of grams of trans fat.
///
/// https://schema.org/transFatContent
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TransFatContentProperty {
    #[cfg(any(any(feature = "mass-schema", feature = "general-schema-section"), doc))]
    Mass(Mass),
}
