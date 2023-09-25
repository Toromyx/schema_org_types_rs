use super::*;
/// The insurance plans that cover this drug.
///
/// <https://schema.org/includedInHealthInsurancePlan>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
