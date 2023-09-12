use super::*;
/// The expected salary upon completing the training.
///
/// https://schema.org/salaryUponCompletion
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SalaryUponCompletionProperty {
    #[cfg(any(
        feature = "monetary-amount-distribution-schema",
        feature = "general-schema-section"
    ))]
    MonetaryAmountDistribution(MonetaryAmountDistribution),
}
