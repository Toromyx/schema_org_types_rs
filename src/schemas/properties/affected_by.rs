use super::*;
/// Drugs that affect the test's results.
///
/// https://schema.org/affectedBy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AffectedByProperty {
    #[cfg(any(feature = "drug-schema", feature = "health-lifesci-schema-section"))]
    Drug(Drug),
}
