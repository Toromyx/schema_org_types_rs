use super::*;
/// <https://schema.org/hasCourseInstance>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasCourseInstanceProperty {
    #[cfg(any(
        any(feature = "course-instance-schema", feature = "general-schema-section"),
        doc
    ))]
    CourseInstance(CourseInstance),
}
