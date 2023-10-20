use super::*;
/// <https://schema.org/sdPublisher>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SdPublisherProperty {
	#[cfg(any(
		any(feature = "organization-schema", feature = "general-schema-section"),
		doc
	))]
	Organization(Organization),
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
}
