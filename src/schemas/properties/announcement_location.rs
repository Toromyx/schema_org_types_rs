use super::*;
/// Indicates a specific [[CivicStructure]] or [[LocalBusiness]] associated with the SpecialAnnouncement. For example, a specific testing facility or business with special opening hours. For a larger geographic region like a quarantine of an entire region, use [[spatialCoverage]].
///
/// https://schema.org/announcementLocation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AnnouncementLocationProperty {
    #[cfg(any(feature = "civic-structure-schema", feature = "general-schema-section"))]
    CivicStructure(CivicStructure),
    #[cfg(any(feature = "local-business-schema", feature = "general-schema-section"))]
    LocalBusiness(LocalBusiness),
}
