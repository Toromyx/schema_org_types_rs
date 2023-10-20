use super::*;
/// <https://schema.org/announcementLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
