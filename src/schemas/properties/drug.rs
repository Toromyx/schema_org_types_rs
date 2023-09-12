use super::*;
/// Specifying a drug or medicine used in a medication procedure.
///
/// https://schema.org/drug
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DrugProperty {
    #[cfg(any(feature = "drug-schema", feature = "health-lifesci-schema-section"))]
    Drug(Drug),
}
