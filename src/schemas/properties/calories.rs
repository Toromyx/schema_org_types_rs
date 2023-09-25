use super::*;
/// The number of calories.
///
/// <https://schema.org/calories>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CaloriesProperty {
    #[cfg(any(
        any(feature = "energy-schema", feature = "general-schema-section"),
        doc
    ))]
    Energy(Energy),
}
