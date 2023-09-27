use super::*;
/// <https://schema.org/hasCourse>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasCourseProperty {
    #[cfg(any(
        any(feature = "course-schema", feature = "general-schema-section"),
        doc
    ))]
    Course(Course),
}
