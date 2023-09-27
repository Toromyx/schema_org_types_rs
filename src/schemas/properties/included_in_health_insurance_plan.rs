use super::*;
/// <https://schema.org/includedInHealthInsurancePlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludedInHealthInsurancePlanProperty {
    #[cfg(any(
        any(
            feature = "health-insurance-plan-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    HealthInsurancePlan(HealthInsurancePlan),
}
