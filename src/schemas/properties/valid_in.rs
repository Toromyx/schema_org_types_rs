use super::*;
/// <https://schema.org/validIn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ValidInProperty {
	#[cfg(any(
		any(
			feature = "administrative-area-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	AdministrativeArea(AdministrativeArea),
}
