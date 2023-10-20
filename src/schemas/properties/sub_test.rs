use super::*;
/// <https://schema.org/subTest>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubTestProperty {
	#[cfg(any(
		any(
			feature = "medical-test-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTest(MedicalTest),
}
