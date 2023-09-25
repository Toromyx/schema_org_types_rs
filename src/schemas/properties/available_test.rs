use super::*;
/// A diagnostic test or procedure offered by this lab.
///
/// <https://schema.org/availableTest>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AvailableTestProperty {
    #[cfg(any(
        any(
            feature = "medical-test-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalTest(MedicalTest),
}
