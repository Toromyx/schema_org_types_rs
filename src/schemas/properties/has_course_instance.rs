use super::*;
/// An offering of the course at a specific time and place or through specific media or mode of study or to a specific section of students.
///
/// https://schema.org/hasCourseInstance
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasCourseInstanceProperty {
    #[cfg(any(feature = "course-instance-schema", feature = "general-schema-section"))]
    CourseInstance(CourseInstance),
}
