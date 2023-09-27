use super::*;
/// <https://schema.org/educationalAlignment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EducationalAlignmentProperty {
    #[cfg(any(
        any(
            feature = "alignment-object-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AlignmentObject(AlignmentObject),
}
