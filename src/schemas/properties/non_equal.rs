use super::*;
/// This ordering relation for qualitative values indicates that the subject is not equal to the object.
///
/// <https://schema.org/nonEqual>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NonEqualProperty {
    #[cfg(any(
        any(
            feature = "qualitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QualitativeValue(QualitativeValue),
}
