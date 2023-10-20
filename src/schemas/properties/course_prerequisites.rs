use super::*;
/// <https://schema.org/coursePrerequisites>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
