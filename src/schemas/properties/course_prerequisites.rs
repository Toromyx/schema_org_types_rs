use super::*;
/// Requirements for taking the Course. May be completion of another [[Course]] or a textual description like "permission of instructor". Requirements may be a pre-requisite competency, referenced using [[AlignmentObject]].
///
/// https://schema.org/coursePrerequisites
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoursePrerequisitesProperty {
    #[cfg(any(
        feature = "alignment-object-schema",
        feature = "general-schema-section"
    ))]
    AlignmentObject(AlignmentObject),
    #[cfg(any(feature = "course-schema", feature = "general-schema-section"))]
    Course(Course),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
