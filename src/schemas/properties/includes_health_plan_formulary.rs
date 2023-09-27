use super::*;
/// <https://schema.org/includesHealthPlanFormulary>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
