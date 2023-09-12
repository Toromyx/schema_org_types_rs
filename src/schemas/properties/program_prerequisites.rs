use super::*;
/// Prerequisites for enrolling in the program.
///
/// https://schema.org/programPrerequisites
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProgramPrerequisitesProperty {
    #[cfg(any(
        feature = "alignment-object-schema",
        feature = "general-schema-section"
    ))]
    AlignmentObject(AlignmentObject),
    #[cfg(any(feature = "course-schema", feature = "general-schema-section"))]
    Course(Course),
    #[cfg(any(
        feature = "educational-occupational-credential-schema",
        feature = "pending-schema-section"
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
