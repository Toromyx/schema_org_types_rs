use super::*;
/// This ordering relation for qualitative values indicates that the subject is equal to the object.
///
/// https://schema.org/equal
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EqualProperty {
    #[cfg(any(
        feature = "qualitative-value-schema",
        feature = "general-schema-section"
    ))]
    QualitativeValue(QualitativeValue),
}
