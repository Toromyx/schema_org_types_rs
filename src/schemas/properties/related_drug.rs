use super::*;
/// Any other drug related to this one, for example commonly-prescribed alternatives.
///
/// https://schema.org/relatedDrug
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelatedDrugProperty {
    #[cfg(any(feature = "drug-schema", feature = "health-lifesci-schema-section"))]
    Drug(Drug),
}
