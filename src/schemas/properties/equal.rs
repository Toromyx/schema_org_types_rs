use super::*;
/// <https://schema.org/equal>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EqualProperty {
    #[cfg(any(
        any(
            feature = "qualitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QualitativeValue(QualitativeValue),
}
