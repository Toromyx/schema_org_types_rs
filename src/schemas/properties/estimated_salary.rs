use super::*;
/// An estimated salary for a job posting or occupation, based on a variety of variables including, but not limited to industry, job title, and location. Estimated salaries  are often computed by outside organizations rather than the hiring organization, who may not have committed to the estimated value.
///
/// https://schema.org/estimatedSalary
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EstimatedSalaryProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(
        feature = "monetary-amount-distribution-schema",
        feature = "general-schema-section"
    ))]
    MonetaryAmountDistribution(MonetaryAmountDistribution),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}
