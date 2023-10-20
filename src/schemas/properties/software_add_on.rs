use super::*;
/// <https://schema.org/softwareAddOn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SoftwareAddOnProperty {
	#[cfg(any(
		any(
			feature = "software-application-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	SoftwareApplication(SoftwareApplication),
}
