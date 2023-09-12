use super::*;
/// The class of drug this belongs to (e.g., statins).
///
/// https://schema.org/drugClass
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DrugClassProperty {
    #[cfg(any(
        feature = "drug-class-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DrugClass(DrugClass),
}
