use super::*;
/// Prerequisites for enrolling in the program.
///
/// <https://schema.org/programPrerequisites>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ProgramPrerequisitesProperty {
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
    #[cfg(any(
        any(
            feature = "educational-occupational-credential-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
