use super::*;
/// Formularies covered by this plan.
///
/// <https://schema.org/includesHealthPlanFormulary>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludesHealthPlanFormularyProperty {
    #[cfg(any(
        any(
            feature = "health-plan-formulary-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    HealthPlanFormulary(HealthPlanFormulary),
}
