use super::*;
/// A sub property of participant. The real estate agent involved in the action.
///
/// https://schema.org/realEstateAgent
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RealEstateAgentProperty {
    #[cfg(any(
        any(
            feature = "real-estate-agent-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    RealEstateAgent(RealEstateAgent),
}
