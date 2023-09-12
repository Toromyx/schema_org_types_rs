use super::*;
/// The number of grams of fat.
///
/// https://schema.org/fatContent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FatContentProperty {
    #[cfg(any(feature = "mass-schema", feature = "general-schema-section"))]
    Mass(Mass),
}
