use super::*;
/// The number of grams of sugar.
///
/// https://schema.org/sugarContent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SugarContentProperty {
    #[cfg(any(feature = "mass-schema", feature = "general-schema-section"))]
    Mass(Mass),
}
