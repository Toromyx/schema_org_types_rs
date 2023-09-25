use super::*;
/// A diagnostic test that can identify this sign.
///
/// https://schema.org/identifyingTest
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IdentifyingTestProperty {
    #[cfg(any(
        any(
            feature = "medical-test-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalTest(MedicalTest),
}
