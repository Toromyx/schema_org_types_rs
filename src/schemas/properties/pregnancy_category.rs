use super::*;
/// Pregnancy category of this drug.
///
/// https://schema.org/pregnancyCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PregnancyCategoryProperty {
    #[cfg(any(
        feature = "drug-pregnancy-category-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DrugPregnancyCategory(DrugPregnancyCategory),
}
