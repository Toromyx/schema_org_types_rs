use super::*;
/// <https://schema.org/identifyingExam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
