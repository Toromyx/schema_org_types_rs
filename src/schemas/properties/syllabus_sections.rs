use super::*;
/// Indicates (typically several) Syllabus entities that lay out what each section of the overall course will cover.
///
/// https://schema.org/syllabusSections
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SyllabusSectionsProperty {
    #[cfg(any(
        any(feature = "syllabus-schema", feature = "pending-schema-section"),
        doc
    ))]
    Syllabus(Syllabus),
}
