use super::*;
/// Networks covered by this plan.
///
/// https://schema.org/includesHealthPlanNetwork
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludesHealthPlanNetworkProperty {
    #[cfg(any(
        any(
            feature = "health-plan-network-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    HealthPlanNetwork(HealthPlanNetwork),
}
