use super::*;
/// The media network(s) whose content is broadcast on this station.
///
/// https://schema.org/broadcastAffiliateOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BroadcastAffiliateOfProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}
