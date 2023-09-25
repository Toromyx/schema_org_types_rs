use super::*;
/// A physical examination that can identify this sign.
///
/// <https://schema.org/identifyingExam>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IdentifyingExamProperty {
    #[cfg(any(
        any(
            feature = "physical-exam-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    PhysicalExam(PhysicalExam),
}
