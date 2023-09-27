use super::*;
/// <https://schema.org/announcementLocation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AnnouncementLocationProperty {
    #[cfg(any(
        any(feature = "civic-structure-schema", feature = "general-schema-section"),
        doc
    ))]
    CivicStructure(CivicStructure),
    #[cfg(any(
        any(feature = "local-business-schema", feature = "general-schema-section"),
        doc
    ))]
    LocalBusiness(LocalBusiness),
}
