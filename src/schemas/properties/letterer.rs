use super::*;
/// <https://schema.org/letterer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LettererProperty {
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
}
