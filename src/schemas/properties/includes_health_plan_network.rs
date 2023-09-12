use super::*;
/// Networks covered by this plan.
///
/// https://schema.org/includesHealthPlanNetwork
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludesHealthPlanNetworkProperty {
    #[cfg(any(
        feature = "health-plan-network-schema",
        feature = "pending-schema-section"
    ))]
    HealthPlanNetwork(HealthPlanNetwork),
}
