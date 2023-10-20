use super::*;
/// <https://schema.org/musicBy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MusicByProperty {
	#[cfg(any(
		any(feature = "music-group-schema", feature = "general-schema-section"),
		doc
	))]
	MusicGroup(MusicGroup),
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
}
