use super::*;
/// <https://schema.org/opponent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpponentProperty {
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
}
