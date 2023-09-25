use super::*;
/// The expected salary upon completing the training.
///
/// <https://schema.org/salaryUponCompletion>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SalaryUponCompletionProperty {
    #[cfg(any(
        any(
            feature = "monetary-amount-distribution-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MonetaryAmountDistribution(MonetaryAmountDistribution),
}
