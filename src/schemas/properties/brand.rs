use super::*;
/// <https://schema.org/brand>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BrandProperty {
	#[cfg(any(any(feature = "brand-schema", feature = "general-schema-section"), doc))]
	Brand(Brand),
	#[cfg(any(
		any(feature = "organization-schema", feature = "general-schema-section"),
		doc
	))]
	Organization(Organization),
}
