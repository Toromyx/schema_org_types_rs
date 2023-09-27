use super::*;
/// <https://schema.org/suitableForDiet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuitableForDietProperty {
    #[cfg(any(
        any(feature = "restricted-diet-schema", feature = "general-schema-section"),
        doc
    ))]
    RestrictedDiet(RestrictedDiet),
}
