use super::*;
/// <https://schema.org/suitableForDiet>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SuitableForDietProperty {
    #[cfg(any(
        any(feature = "restricted-diet-schema", feature = "general-schema-section"),
        doc
    ))]
    RestrictedDiet(RestrictedDiet),
}
