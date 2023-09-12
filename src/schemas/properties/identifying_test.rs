use super::*;
/// A diagnostic test that can identify this sign.
///
/// https://schema.org/identifyingTest
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IdentifyingTestProperty {
    #[cfg(any(
        feature = "medical-test-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTest(MedicalTest),
}
