use super::*;
/// Pregnancy category of this drug.
///
/// https://schema.org/pregnancyCategory
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PregnancyCategoryProperty {
    #[cfg(any(
        any(
            feature = "drug-pregnancy-category-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DrugPregnancyCategory(DrugPregnancyCategory),
}
