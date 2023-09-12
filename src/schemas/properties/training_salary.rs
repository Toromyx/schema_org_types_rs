use super::*;
/// The estimated salary earned while in the program.
///
/// https://schema.org/trainingSalary
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TrainingSalaryProperty {
    #[cfg(any(
        feature = "monetary-amount-distribution-schema",
        feature = "general-schema-section"
    ))]
    MonetaryAmountDistribution(MonetaryAmountDistribution),
}
