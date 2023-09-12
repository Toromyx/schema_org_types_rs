use super::*;
/// An alignment to an established educational framework.
///
/// This property should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource [[teaches]] or [[assesses]] a competency.
///
/// https://schema.org/educationalAlignment
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EducationalAlignmentProperty {
    #[cfg(any(
        feature = "alignment-object-schema",
        feature = "general-schema-section"
    ))]
    AlignmentObject(AlignmentObject),
}
