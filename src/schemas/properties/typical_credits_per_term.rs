use super::*;
/// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
///
/// https://schema.org/typicalCreditsPerTerm
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypicalCreditsPerTermProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(
        feature = "structured-value-schema",
        feature = "general-schema-section"
    ))]
    StructuredValue(StructuredValue),
}
