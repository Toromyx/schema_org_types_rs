use super::*;
/// The estimated salary earned while in the program.
///
/// <https://schema.org/trainingSalary>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TrainingSalaryProperty {
    #[cfg(any(
        any(
            feature = "monetary-amount-distribution-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MonetaryAmountDistribution(MonetaryAmountDistribution),
}
