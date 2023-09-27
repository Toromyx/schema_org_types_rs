use super::*;
/// <https://schema.org/educationalAlignment>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
