use super::*;
/// governmentBenefitsInfo provides information about government benefits associated with a SpecialAnnouncement.
///
/// https://schema.org/governmentBenefitsInfo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GovernmentBenefitsInfoProperty {
    #[cfg(any(
        feature = "government-service-schema",
        feature = "general-schema-section"
    ))]
    GovernmentService(GovernmentService),
}
