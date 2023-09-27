use super::*;
/// <https://schema.org/syllabusSections>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SyllabusSectionsProperty {
    #[cfg(any(
        any(feature = "syllabus-schema", feature = "pending-schema-section"),
        doc
    ))]
    Syllabus(Syllabus),
}
