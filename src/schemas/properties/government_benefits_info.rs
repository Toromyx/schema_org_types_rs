use super::*;
/// <https://schema.org/governmentBenefitsInfo>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GovernmentBenefitsInfoProperty {
    #[cfg(any(
        any(
            feature = "government-service-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    GovernmentService(GovernmentService),
}
