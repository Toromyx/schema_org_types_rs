use super::*;
/// <https://schema.org/penciler>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PencilerProperty {
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
}
