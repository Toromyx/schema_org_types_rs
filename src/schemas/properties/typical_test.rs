use super::*;
/// A medical test typically performed given this condition.
///
/// https://schema.org/typicalTest
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypicalTestProperty {
    #[cfg(any(
        feature = "medical-test-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTest(MedicalTest),
}