use super::*;
/// <https://schema.org/usesDevice>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum UsesDeviceProperty {
	#[cfg(any(
		any(
			feature = "medical-device-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalDevice(MedicalDevice),
}
