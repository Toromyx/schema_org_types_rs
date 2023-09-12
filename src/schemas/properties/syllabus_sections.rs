use super::*;
/// Indicates (typically several) Syllabus entities that lay out what each section of the overall course will cover.
///
/// https://schema.org/syllabusSections
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SyllabusSectionsProperty {
    #[cfg(any(feature = "syllabus-schema", feature = "pending-schema-section"))]
    Syllabus(Syllabus),
}
