use super::*;
/// An alignment to an established educational framework.
///
/// This property should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource [[teaches]] or [[assesses]] a competency.
///
/// https://schema.org/educationalAlignment
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
