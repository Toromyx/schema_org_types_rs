use super::*;
/// <https://schema.org/greaterOrEqual>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GreaterOrEqualProperty {
    #[cfg(any(
        any(
            feature = "qualitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QualitativeValue(QualitativeValue),
}
