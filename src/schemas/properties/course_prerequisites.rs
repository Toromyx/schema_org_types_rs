use super::*;
/// Requirements for taking the Course. May be completion of another [[Course]] or a textual description like "permission of instructor". Requirements may be a pre-requisite competency, referenced using [[AlignmentObject]].
///
/// https://schema.org/coursePrerequisites
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CoursePrerequisitesProperty {
    #[cfg(any(
        any(
            feature = "alignment-object-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AlignmentObject(AlignmentObject),
    #[cfg(any(
        any(feature = "course-schema", feature = "general-schema-section"),
        doc
    ))]
    Course(Course),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
