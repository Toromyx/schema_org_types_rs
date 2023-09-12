use super::*;
/// The insurance plans that cover this drug.
///
/// https://schema.org/includedInHealthInsurancePlan
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludedInHealthInsurancePlanProperty {
    #[cfg(any(
        feature = "health-insurance-plan-schema",
        feature = "pending-schema-section"
    ))]
    HealthInsurancePlan(HealthInsurancePlan),
}
