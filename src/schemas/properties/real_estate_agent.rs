use super::*;
/// A sub property of participant. The real estate agent involved in the action.
///
/// https://schema.org/realEstateAgent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealEstateAgentProperty {
    #[cfg(any(
        feature = "real-estate-agent-schema",
        feature = "general-schema-section"
    ))]
    RealEstateAgent(RealEstateAgent),
}
