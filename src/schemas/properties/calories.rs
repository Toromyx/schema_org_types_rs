use super::*;
/// The number of calories.
///
/// https://schema.org/calories
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CaloriesProperty {
    #[cfg(any(feature = "energy-schema", feature = "general-schema-section"))]
    Energy(Energy),
}